use crate::{
  Transform, 
  Node,
  CompoundExpressionNode,
  CompoundExpressionNodeChild,
  TextCallNode,
  TextCallNodeContent,
  TransformContext,
};


pub struct TransformText;

impl Transform for TransformText {
  #[allow(unused_variables)]
  fn pre_transform(&self, node: &mut Node, _ctx: &mut TransformContext) {
    return;
  }

  fn post_transform(&self, node: &mut Node, _ctx: &mut TransformContext) {
    let children = get_node_children(node);
    if let Some(children) = children {
      process_children(children);
    }
  }
}

fn get_node_children(node: &mut Node) -> Option<&mut Vec<Node>> {
  match node {
    Node::ElementNode(n) => Some(&mut n.children),
    Node::Root(n) => Some(&mut n.children),
    _ => None
  }
}

// 1. bar {{ foo }} baz 这种情况下要把, 要把3个节点合并为一个节点CompoundNode。
// 2. 检查父元素如果只有单个子元素直接返回即可。
// 3. 如果父元素有多个子元素，那么需要为每个文本节点, 创建一个TextCall。

fn process_children(children: &mut Vec<Node>) {
  let mut has_text = false;
  let mut has_compound = false;

  for i in 0..children.len() {

    let child =&mut children[i];
    let is_text: Option<CompoundExpressionNodeChild> = child.clone().into();
    if let Some(prev ) = is_text {
      has_text = true;
      let mut j = i + 1;
      loop {
        if j >= children.len() {
          break;
        }
        let next = &mut children[j];
        let is_text: Option<CompoundExpressionNodeChild> = next.clone().into();

        if let Some(next) = is_text {
          if !has_compound {
            has_compound = true;
            children[i] = Node::CompoundExpression(CompoundExpressionNode {
              children: vec![prev.clone()]
            });
          }
          if let Node::CompoundExpression(n) = &mut children[i] {
            n.children.push(CompoundExpressionNodeChild::RawText(" + ".to_string()));
            n.children.push(next);
            j += 1;
          }
          children.remove(j); // 删除节点, j要回退1
        } else {
          has_compound = false;
          break;
        }
      }
    }
  }

  if !has_text || children.len() == 1 {
    return;
  }

  for child in children {
    let text_call_node_child: Option<TextCallNodeContent> = child.into();
    if let Some(text) = text_call_node_child {

      *child = Node::TextCall(TextCallNode {
        content: text,
        loc: Default::default()
      })
    }
  }
}

impl From<Node> for Option<CompoundExpressionNodeChild> {
  fn from(node: Node) -> Self {
    match node {
      Node::Text(n) => {
        Some(CompoundExpressionNodeChild::TextNode(n))
      },
      Node::Interpolation(n) => {
        Some(CompoundExpressionNodeChild::InterpolationNode(n))
      },
      _ => None
    }
  }
}

impl From<&mut Node> for Option<TextCallNodeContent> {
  fn from(node: &mut Node) -> Self {
    match node {
      Node::Text(n) => {
        Some(TextCallNodeContent::TextNode(n.clone()))
      },
      Node::Interpolation(n) => {
        Some(TextCallNodeContent::InterpolationNode(n.clone()))
      },
      Node::CompoundExpression(n) => {
        Some(TextCallNodeContent::CompoundExpressionNode(n.clone()))
      },
      _ => None
    }
  }
}