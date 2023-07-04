mod ast;
pub use ast::*;
mod parse;
pub use parse::*;
mod input;
pub use input::*;
mod error;
pub use error::*;
mod options;
pub use options::*;
mod transform;
pub use transform::*;
mod utils;
pub use utils::*;
mod code_gen;
pub use code_gen::*;
mod patch_flags;
pub use patch_flags::*;


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_transform() {
    // let mut parser = Parser::new("<div><span/></div>");
    // let mut ast = parser.parse();
    
    // let mut transform_runner = TransformRunner::new(
    //   vec![
    //     Box::new(TransformElement{}),
    //     Box::new(TransformText{})
    //   ],
    //   TransformContext::new("hello.vue".to_string()),
    // );
    // transform_runner.travel_node(&mut ast);
  }
}