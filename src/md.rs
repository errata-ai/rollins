//! .
extern crate syntect;

use std::io::{BufRead, BufReader};
use std::str::FromStr;

use syntect::parsing::{ParseState, ScopeStack, SyntaxDefinition};
use syntect::highlighting::ScopeSelectors;
use syntect::easy::ScopeRegionIterator;

use ast::{MarkupSelectors, Node, NodeState};

fn should_terminate(state: &mut NodeState, text: &str) -> bool {
    let blank = state.line.chars().all(|c| c == '\n');
    if (state.block == 1 && blank) || state.block == 2 && text.ends_with("\n") {
        return true;
    }
    return false;
}

fn make_node(
    state: &mut NodeState,
    stack: &ScopeStack,
    sel: &MarkupSelectors,
    text: &str,
) -> Option<Node> {
    let sliced = stack.as_slice();
    let scope = sliced.last().unwrap().to_string();
    // let inline = sel.inline.does_match(sliced).is_some();

    if should_terminate(state, text)
    {
        // We've found the end of a block (as indicated by the
        // scope change).
        let mut src_line = state.index - state.context.lines().count();
        if state.block == 2 && !state.text.ends_with("\n") {
            // TODO: Why is this necessary?
            src_line += 1;
        }
        return Some(Node {
            context: state.context.clone(),
            line: src_line,
            scope: sel.selector_to_scope(&state.top.clone()),
        });
    }

    if state.block > 0 || sel.is_block(scope.clone()) {
        state.context.push_str(&text);
        if !(
                scope.starts_with("punctuation.definition") ||
                scope.contains("underline.link") ||
                scope.starts_with("constant.other")
        ) {
            // TODO: Maybe filter code spans?
            state.text.push_str(&text);
        }
        if state.block == 0 {
            // We've found the start of a block -- so we save the
            // top-most scope to push into the AST.
            state.top = scope.clone();
            state.block = sel.block_type(scope);
        }
    }

    None
}

pub fn md_to_ast(syntax: &SyntaxDefinition, text: &str) -> Vec<Node> {
    let sel: MarkupSelectors = MarkupSelectors {
        paragraph: "meta.paragraph.markdown".to_owned(),
        list: "punctuation.definition.list_item.markdown".to_owned(),
        heading: "punctuation.definition.heading.begin.markdown".to_owned(),
        quote: "punctuation.definition.blockquote.markdown".to_owned(),
        cell: "meta.table.markdown".to_owned(),
        inline: ScopeSelectors::from_str(
            "markup.raw.inline, markup.bold, markup.italic, markup.quote, punctuation.definition, punctuation.terminator, meta.link, constant.other, constant.character",
        ).unwrap(),
    };

    let mut pstate = ParseState::new(syntax);
    let mut nstate = NodeState::default();
    let mut reader = BufReader::new(text.as_bytes());
    let mut line = String::new();
    let mut stack = ScopeStack::new();

    let mut ast: Vec<Node> = Vec::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        nstate.index += 1;
        if line.is_empty() {
            continue;
        }

        nstate.line = line.clone();
        for (s, op) in ScopeRegionIterator::new(&pstate.parse_line(&line), &line) {
            stack.apply(op);
            if s.is_empty() {
                continue;
            } else if let Some(n) = make_node(&mut nstate, &stack, &sel, s) {
                if !n.context.chars().all(|c| c.is_whitespace()) {
                    ast.push(n);
                }
                nstate.clear();
            }
        }
        line.clear();
    }

    if !nstate.context.is_empty() {
        ast.push(Node {
            context: nstate.context.clone(),
            line: nstate.index - nstate.context.lines().count(),
            scope: sel.selector_to_scope(&nstate.top.clone()),
        });
    }

    ast
}
