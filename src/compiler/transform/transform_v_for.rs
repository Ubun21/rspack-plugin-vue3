use crate::{Node, Transform, TransformContext};

pub struct TransformVFor;

impl Transform for TransformVFor {
  fn pre_transform(&self, node: &mut Node, ctx: &mut TransformContext) {
    return;
  }

  fn post_transform(&self, node: &mut Node, ctx: &mut TransformContext) {
    return;
  } 
}
