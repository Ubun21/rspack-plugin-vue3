use std::{fs::File};
use std::io::Read;
use std::path::Path;
use regex::Regex;

use crate::{Node};

pub fn read_file_sync(path: &Path) -> String {
  let mut file = File::open(path).unwrap();
  let mut buffer = String::new();
  file.read_to_string(&mut buffer).unwrap();
  buffer.to_string()
}

pub fn is_text_node(node: &Node) -> bool {
  match node {
    Node::Text(_) | Node::Interpolation(_) => true,
    _ => false
  }
}

pub fn is_compound_expression(node: &Node) -> bool {
  match node {
    Node::CompoundExpression(_) => true,
    _ => false
  }
}

pub fn is_simple_identifier(content: &str) -> bool {
  let reg = Regex::new(r"^\d|[^\$\w]").unwrap();
  !reg.is_match(content)
}

const DIRECTIVES: [&str; 15] = [
    "bind", "cloak",  "else-if", "else", "for", "html", "if", 
    "model", "on", "once", "pre", "show", "slot", "text", "memo"
];

const GLOBALS: [&str; 25] = [
    "Infinity", "undefined", "NaN", "isFinite", "parseFloat", "parseInt", 
    "decodeURI",  "decodeURIComponent", "encodeURI", "encodeURIComponent", 
    "Math", "Number", "Date", "Array", "Object", "Boolean", "String", "RegExp",
    "Map", "Set", "JSON", "Intl", "globalThis", "arguments", "console",
];

pub fn is_build_in_directive(name: &str) -> bool {
  DIRECTIVES.contains(&name)
}

pub fn is_global_white_list(name: &str) -> bool {
  GLOBALS.contains(&name)
}
