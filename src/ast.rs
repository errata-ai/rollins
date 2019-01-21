//! This file contains definitions and logic related to the `rollins` AST.
use syntect::highlighting::ScopeSelectors;


#[derive(Serialize, Deserialize)]
pub struct Node {
    pub context: String,
    pub line: usize,
    pub scope: String,
}

pub struct MarkupSelectors {
    pub paragraph: String,
    pub heading: String,
    pub list: String,
    pub quote: String,
    pub cell: String,
    pub inline: ScopeSelectors,
}

impl MarkupSelectors {
    pub fn selector_to_scope(&self, selector: &str) -> String {
        if selector == self.paragraph {
            return String::from("paragraph");
        } else if selector == self.heading {
            return String::from("heading");
        } else if selector == self.list {
            return String::from("list");
        } else if selector == self.cell {
            return String::from("cell");
        } else {
            return String::from("blockquote");
        }
    }

    pub fn is_block(&self, sel: String) -> bool {
        if sel == self.paragraph || sel == self.quote || sel == self.list || sel == self.cell
            || sel == self.heading
        {
            return true;
        }
        return false;
    }

    pub fn block_type(&self, sel: String) -> usize {
        if sel == self.paragraph || sel == self.quote || sel == self.list {
            return 1;
        }
        return 2;
    }
}

#[derive(Debug, Default)]
pub struct NodeState {
    pub context: String,
    pub block: usize,
    pub text: String,
    pub top: String,
    pub index: usize,
    pub line: String,
}

impl NodeState {
    pub fn clear(&mut self) {
        self.context.clear();
        self.text.clear();
        self.block = 0;
    }
}
