use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde_json::to_string;
use syntect::parsing::SyntaxSet;
use ast::Node;
use md::md_to_ast;

pub fn file_to_string(path: &str) -> Option<String> {
    let p = Path::new(path);
    let mut contents = String::new();
    let display = p.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&p) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    match file.read_to_string(&mut contents) {
        Err(_) => None,
        Ok(_) => {
            return Some(contents);
        }
    }
}

pub fn read_test(path: &str, ext: &str) -> Option<String> {
    if let Some(c) = file_to_string(path) {
        let ast = make_ast(&c, &ext).unwrap();
        return Some(to_string(&ast).unwrap());
    }
    None
}

pub fn make_ast(contents: &str, ext: &str) -> Option<Vec<Node>> {
    let ps = SyntaxSet::load_defaults_newlines();

    /*
    match env::var("ROLLINS_SEARCH") {
        Ok(val) => {
            ps.load_syntaxes(val, true).unwrap();
            ps.link_syntaxes();
        }
        Err(e) => panic!("couldn't interpret {}: {}", "ROLLINS_SEARCH", e),
    }*/

    if let Some(s) = ps.find_syntax_by_extension(ext) {
        return Some(md_to_ast(s, contents));
    }
    None
}
