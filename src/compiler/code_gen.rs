use crate::{Node, TextNode};
pub struct CodeGen<'a> {
  pub code: String,
  pub ast: &'a Node
}

impl<'a> CodeGen<'a> {
  pub fn new(ast: &'a Node) -> Self {
    Self {
      code: String::new(),
      ast
    }
  }

  pub fn generate(&mut self){
  }

  pub fn generate_text(&mut self, node: &TextNode) {
    self.code.push_str(node.content.as_str());
  }  
}
