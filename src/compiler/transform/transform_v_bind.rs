use crate::{
  DirectiveTransform, 
  DirectiveTransformRes, 
  DirectiveProp, 
  TransformContext, ExpressionNode, Property, JsChildNode
};

pub struct TransformBind;

impl DirectiveTransform for TransformBind {
    fn transform(
      &self, 
      dir: &mut DirectiveProp, 
      _node: &mut crate::Node, 
      _ctx: &mut TransformContext) -> DirectiveTransformRes {
        let DirectiveProp { 
          arg, 
          exp, .. } = dir;

        if let Some(ExpressionNode::SimpleExpressionNode(exp)) = exp {
          if exp.content == "" {
            todo!("emit error ErrorCodes.X_V_BIND_NO_EXPRESSION");
          }
        } else {
          todo!("emit error ErrorCodes.X_V_BIND_NO_EXPRESSION");
        }

        DirectiveTransformRes { 
          properties: vec![
            Property {
              key: arg.clone().unwrap(),
              value: Box::new(JsChildNode::ExpressionNode(exp.clone().unwrap()))
            }
          ], 
          need_runtime: false 
        }
    }
}
