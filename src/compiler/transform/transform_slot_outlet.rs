use crate::{
  Transform,
  Node,
  RenderSlotCallArgs,
  CodeGenType,
  ElementNodeCodeGen,
  RenderSlotCall,
  TransformContext,
};

pub struct Slot {}

impl Transform for Slot {
  fn pre_transform(&self, node: &mut Node, _ctx: &mut TransformContext) {
    if let Node::ElementNode(n) = node {
      let mut args: Vec<RenderSlotCallArgs> = vec![];
      args.push(RenderSlotCallArgs::RawText("slot".to_string()));
      args.push(RenderSlotCallArgs::RawText("default".to_string()));

      let code_gen = ElementNodeCodeGen {
        gen_type: CodeGenType::SlotOutletNodeCodeGen,
        render_slot_call: Some(RenderSlotCall {
          call: "renderSlot".to_string(),
          args
        }),
        ..Default::default()
      };
      n.code_gen = Some(code_gen);
    }
  }

  fn post_transform(&self, _node: &mut Node, _ctx: &mut TransformContext) {
    return;
  }
}
