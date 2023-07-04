use crate::{Node, Transform, TransformContext, Prop, IfNode};

pub struct TransformVIf;

impl Transform for TransformVIf {
  fn pre_transform(&self, node: &mut Node, ctx: &mut TransformContext) {
    if let Node::ElementNode(n) = node {
      for p in n.props.clone().iter() {
        if let Prop::Directive(d) = p {
          if d.name == "if" {
            let if_branch = node.create_if_branch_by_element_base(d.clone());
            let if_node = IfNode { 
              branches: vec![if_branch],
              loc: Default::default()
            };
            node.replace_node(Node::If(if_node))
          } else {
            if let Some(parent) = ctx.parent.clone() {
              let mut slibing = parent.borrow_mut();
              let slibing = slibing.children_mut();
              let mut index = slibing.borrow_mut().iter().position(|x| *x == node.clone()).unwrap();
              while index >= 0 {
                let mut node = slibing.borrow_mut();
                let node = node.get_mut(index).unwrap();
                if let Node::Text(text) = node {
                  if text.content.trim().is_empty() {
                    slibing.borrow_mut().remove(index);
                  }
                  index -= 1;
                  continue;
                }
  
                if let Node::IFBranch(n) = node {
                  slibing.borrow_mut().remove(index);
                  let branch = node.create_if_branch_by_element_base(d.clone());
                  n.children.push(Node::IFBranch(branch));
                }
                break;
              }
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
