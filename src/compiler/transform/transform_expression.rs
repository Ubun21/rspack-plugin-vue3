use swc_ecma_ast::Ident;

use crate::{
  Transform, Node, TransformContext, SimpleExpressionNode, is_global_white_list, 
  ExpressionNode, CompoundExpressionNodeChild, Prop, is_simple_identifier, ProcessIdentifiers};
pub struct Expression {}

impl Transform for Expression {
  fn pre_transform(&self, node: &mut Node, ctx: &mut TransformContext) {
    if let Node::Interpolation(interpolation) = node {
      match &mut interpolation.content {
        ExpressionNode::SimpleExpressionNode(exp) => {
          process_expression(exp, ctx, false);
        },
        ExpressionNode::CompoundExpressionNode(exp) => {
          for child in &mut exp.children {
            match child {
              CompoundExpressionNodeChild::SimpleExpressionNode(exp) => {
                process_expression(exp, ctx, false);
              },
              _ => {}
            }
          }
        },
      } 
    }

    if let Node::ElementNode(el) = node {
      for prop in &mut el.props {
        if let Prop::Directive(dir) = prop {
          if let Some(ExpressionNode::SimpleExpressionNode(exp)) = &mut dir.exp {
            if dir.name != "on" {
              process_expression(exp, ctx, dir.name == "slot");
            }
          }
        }
      }
    }
  }

  fn post_transform(&self, _node: &mut Node, _ctx: &mut TransformContext) {
    return;
  }
}

pub fn process_expression(
  exp: &mut SimpleExpressionNode,
  _ctx: &mut TransformContext,
  _as_param: bool  
) {
  if exp.content.is_empty() {
    return;
  }

  let raw_str = &mut exp.content;
  if is_simple_identifier(&raw_str) {
    return;
  }

  let mut rewrite_idents = ProcessIdentifiers::new(raw_str);
  rewrite_idents.parse();
  rewrite_idents.rewrite_identifiers();
  let code = rewrite_idents.generate();

  *raw_str = code;
}

pub fn can_rewrite(id: &Ident) -> bool {
  let name = id.sym.to_string();
  if is_global_white_list(&name) {
    return false;
  }
  if &name == "require" {
    return false;
  }
  true
}

pub fn rewrite_identifier(id: &mut String) {
  *id = String::from("ctx.")
}