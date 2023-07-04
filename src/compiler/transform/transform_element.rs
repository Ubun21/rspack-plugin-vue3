use crate::{
  Transform,
  Node,
  TransformContext,
  Prop,
  PropExpression,
  NormalProp,
  ObjectExpression,
  SimpleExpressionNode,
  ExpressionNode,
  SourceLocation,
  ConstantTypes,
  TextPropValue,
  Property,
  JsChildNode,
  ElementTagType,
  ElementNodeBase,
  ElementNodeCodeGen,
  VnodeCall,
  CodeGenType,
  DirectiveProp,
  CallExpression,
  CallExpressionArgument, is_build_in_directive,
};

pub struct TransformElement;

pub struct PatchFlagStatus {
  pub patch_flag: i32,
  pub has_ref: bool,
  pub has_class_binding: bool,
  pub has_style_binding: bool,
  pub has_hydration_event_binding: bool,
  pub has_dynamic_keys: bool,
  pub has_vnode_hook: bool,
  pub dynamic_prop_names: Vec<String>
}

pub struct BuildPropResult {
  pub props: Option<PropExpression>,
  pub directive: Vec<DirectiveProp>,
  pub patch_flag: i32,
  pub dynamic_prop_names: Vec<String>,
  pub should_block: bool
}

impl Transform for TransformElement {
  fn pre_transform(&self, _node: &mut Node, _ctx: &mut TransformContext) {
  }

  fn post_transform(&self, node: &mut Node, _ctx: &mut TransformContext) {
    match node {
      Node::ElementNode(n) => {
        println!("pre_transform: {:?}", n);
        match n.tag_type {
          ElementTagType::SlotOutletNode | ElementTagType::TemplateNode => {
            return;
          },
          _ => {}
        }

        let ElementNodeBase { tag_name, tag_type, props, children,.. } = n;

        let props = build_props(props.clone());
        let props = props.props;

        let code_gen = ElementNodeCodeGen {
          gen_type: get_code_gen_type(tag_type),
          vnode_call: Some(VnodeCall {
            tag: tag_name.clone(),
            props,
            children: children.clone(),
            is_component: false,
            ..Default::default()
          }),
          ..Default::default()
        };
        n.code_gen = Some(code_gen);     
      }
      _ => {}
    }
  }
}

pub fn get_code_gen_type(tag_type: &ElementTagType) -> CodeGenType {
  match tag_type {
    ElementTagType::PlainElementNode => CodeGenType::PlainElementNodeCodeGen,
    ElementTagType::ComponentNode => CodeGenType::ComponentNodeCodeGen,
    ElementTagType::SlotOutletNode => CodeGenType::SlotOutletNodeCodeGen,
    ElementTagType::TemplateNode => CodeGenType::TemplateNodeCodeGen,
  }
}

pub fn build_props(props: Vec<Prop>) -> BuildPropResult {
  let mut properties: Vec<Property> = vec![];
  let mut merge_props: Vec<PropExpression> = vec![];
  let mut runtime_directives: Vec<DirectiveProp> = vec![];

  for prop in props.into_iter() {
    match prop {
      Prop::Normal(p) => {
        let NormalProp { name, value, loc } = p;
        if name == "ref" {
          continue;
        }
        if name == "is" {
          continue;
        }
        let property = create_property(name, value.unwrap(), loc);
        println!("property: {:?}", &property);
        properties.push(property);
      },
      Prop::Directive(d) => {
        let DirectiveProp { name, arg, exp, loc, .. } = d.clone();
        let is_bind = name == "bind";
        let is_v_on = name == "on";

        if name == "slot" {
          continue;
        }

        if name == "once" || name == "memo" {
          continue;
        }

        if arg.is_none() && (is_bind || is_v_on) {
          if exp.is_some() {
            if is_bind {
              push_merge_props(&mut merge_props, &mut properties, None);
              let exp = PropExpression::SimpleExpression(exp.unwrap());
              merge_props.push(exp);
            } else {
              // v-on="obj" -> toHandlers(obj)
              let exp = exp_to_prop_exp_args(exp.unwrap());
              let prop = PropExpression::CallExpression(CallExpression { 
                callee: "TO_HANDLERS".to_string(), 
                arguments: vec![exp]
              });
              push_merge_props(
                &mut merge_props, 
                &mut properties, 
                Some(prop));
            } 
          } else {
            // todo 提交一个错误, 不能没有表达式
          }
          continue;
        }

        if !is_build_in_directive(&name) {
          runtime_directives.push(d.clone());
        }
      },
    }
  }

  let mut prop_exp: Option<PropExpression> = None;

  if merge_props.len() > 0 {
    if merge_props.len() > 1 {
      prop_exp = Some(PropExpression::CallExpression(CallExpression { 
        callee: "MERGE_PROPS".to_string(), 
        arguments: prop_exps_to_call_exp_args(merge_props)
      }));
    } else {
      prop_exp = Some(merge_props[0].clone());
    }
  } else if properties.len() > 0 {
    prop_exp = Some(PropExpression::ObjectExpression(ObjectExpression { 
      properties 
    }));
  }

  BuildPropResult { 
    props: prop_exp, 
    directive: runtime_directives, 
    patch_flag: 0, 
    dynamic_prop_names: vec![], 
    should_block: false
  }
}

pub fn analyze_patch_flag(property: &Property, is_component: bool) -> PatchFlagStatus {
  let mut flags = PatchFlagStatus {
    patch_flag: 0,
    has_ref: false,
    has_class_binding: false,
    has_style_binding: false,
    has_hydration_event_binding: false,
    has_dynamic_keys: false,
    has_vnode_hook: false,
    dynamic_prop_names: vec![],
  };

  let Property { key, .. } = property;

  if let Some(exp) = get_static_exp(key) {
    let name = &exp.content;

    if name == "ref" {
      flags.has_ref = true;
    } else if name == "class" {
      flags.has_class_binding = true;
    } else if name == "style" {
      flags.has_style_binding = true;
    } else if name != "key" && !flags.dynamic_prop_names.contains(name) {
      flags.dynamic_prop_names.push(name.clone());
    }

    if is_component && 
      (name == "class" && name == "style") && 
      !flags.dynamic_prop_names.contains(name) {
        flags.dynamic_prop_names.push(name.clone());
    }
  } else {
    flags.has_dynamic_keys = true;
  }

  flags
}

pub fn get_static_exp(exp: &ExpressionNode) -> Option<&SimpleExpressionNode> {
  match exp {
    ExpressionNode::SimpleExpressionNode(n) => {
      if n.is_static {
        return Some(n);
      }
    },
    _ => {}
  }
  None
}

pub fn prop_exps_to_call_exp_args(exp: Vec<PropExpression>) -> Vec<CallExpressionArgument> {
  let mut args: Vec<CallExpressionArgument> = vec![];
  for e in exp.into_iter() {
    args.push(CallExpressionArgument::PropExpression(e));
  }
  args
}

pub fn push_merge_props(
  merge_props: &mut Vec<PropExpression>, 
  properties: &mut Vec<Property>,
  arg: Option<PropExpression>) {
  if properties.len() > 0 {
    let object_expression = ObjectExpression { properties: properties.clone() };
    let prop_expression = PropExpression::ObjectExpression(object_expression);
    merge_props.push(prop_expression);
    properties.clear();
  }
  if let Some(arg) = arg {
    merge_props.push(arg);
  }
}

pub fn exp_to_prop_exp_args(exp: ExpressionNode) -> CallExpressionArgument {
  CallExpressionArgument::JsChildNode(Box::new(JsChildNode::ExpressionNode(exp)))
}

pub fn create_property(
  name: String, 
  value: TextPropValue, 
  loc: SourceLocation) -> Property {
  let value = 
    ExpressionNode::SimpleExpressionNode(SimpleExpressionNode { 
      content: value.content, 
      is_static: true, 
      constant_type: ConstantTypes::NotConstant, 
      loc,
    });
  let js_child = Box::new(JsChildNode::ExpressionNode(value));

  let key = 
    ExpressionNode::SimpleExpressionNode(SimpleExpressionNode { 
      content: name, 
      is_static: true, 
      constant_type: ConstantTypes::NotConstant, 
      loc: Default::default(),
    });

  let property = Property { 
    key,
    value: js_child,
  };

  property
}
