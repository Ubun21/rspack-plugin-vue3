use crate::{DirectiveTransform, DirectiveTransformRes, DirectiveProp, Node, TransformContext, ExpressionNode, SimpleExpressionNode, ConstantTypes, Expression, CompoundExpressionNode, CompoundExpressionNodeChild, Property, JsChildNode};

pub struct TransformVModel;

impl DirectiveTransform for TransformVModel {
  fn transform(
      &self, dir: 
      &mut DirectiveProp, 
      node: &mut Node, 
      ctx: &mut TransformContext) -> DirectiveTransformRes {
      let DirectiveProp { 
        arg, 
        exp, .. } = dir;

      if exp.is_none() || arg.is_none() {
        todo!("emit error ErrorCodes.X_V_MODEL_NO_EXPRESSION")
      }

      let raw_str = match exp.clone().unwrap() {
        ExpressionNode::SimpleExpressionNode(exp) => {
          exp.content.clone()
        },
        _ => {
          "".to_string()
        }
      };

      let mut simple = SimpleExpressionNode {
        content: "modelValue".to_string(),
        is_static: false,
        constant_type: ConstantTypes::NotConstant,
        loc: Default::default(),
      };
      let prop_name = if let Some(ExpressionNode::SimpleExpressionNode(arg)) = arg {
        arg
      } else {
        &simple
      };

      let event_name = "onUpdate:modelValue".to_string();

      let event_arg = if ctx.is_ts {
        "($event: any)".to_string()
      } else {
        "$event".to_string()
      };
      
      let exp = match exp.clone().unwrap() {
        ExpressionNode::SimpleExpressionNode(exp) => { exp },
        ExpressionNode::CompoundExpressionNode(_exp) => {
          todo!("")
        },
      };

      let assign_exp = CompoundExpressionNode {
        children: vec![
          CompoundExpressionNodeChild::RawText(format!("{} => ((", event_arg)),
          CompoundExpressionNodeChild::SimpleExpressionNode(exp),
          CompoundExpressionNodeChild::RawText(") = $event)".to_string()),
        ]
      };

      let mut props = vec![];
      props.push(Property {
        key: ExpressionNode::SimpleExpressionNode(prop_name.to_owned()),
        value: Box::new(JsChildNode::ExpressionNode(dir.exp.clone().unwrap())),
      });
      props.push(Property { 
        key: ExpressionNode::SimpleExpressionNode(SimpleExpressionNode {
          content: event_name,
          is_static: true,
          constant_type: ConstantTypes::NotConstant,
          loc: Default::default(),
        }),
        value: Box::new(JsChildNode::ExpressionNode(ExpressionNode::CompoundExpressionNode(assign_exp))),
      });

      DirectiveTransformRes { 
        properties: props, 
        need_runtime: false 
      }
  }
}