mod transform_element;
use std::{collections::HashMap, cell::RefCell, rc::Rc};

pub use transform_element::*;
mod transform_text;
pub use transform_text::*;
mod transform_slot_outlet;
pub use transform_slot_outlet::*;
mod transform_v_solt;
pub use transform_v_solt::*;
mod transform_expression;
pub use transform_expression::*;
mod swc_utils;
pub use swc_utils::*;
mod transform_v_bind;
pub use transform_v_bind::*;
mod transform_v_model;
mod transform_v_if;
pub use transform_v_if::*;
mod transform_v_for;
pub use transform_v_for::*;
use crate::{Node, Property, DirectiveProp};


pub trait Transform {
  fn pre_transform(&self, node: &mut Node, ctx: &mut TransformContext);
  fn post_transform(&self, node: &mut Node, ctx: &mut TransformContext);
}

pub struct TransformRunner<'a> {
  pub transforms: Vec<Box<dyn Transform>>,
  pub ctx: TransformContext<'a>,
}

impl<'a> TransformRunner<'a> {
  pub fn new(transforms: Vec<Box<dyn Transform>>, ctx: TransformContext<'a>) -> Self {
    Self {
      transforms,
      ctx: ctx
    }
  }
  
  // pub fn travel_node_mut(&mut self, node: Rc<RefCell<&'a mut Node>>) {
  //   self.travel_node_in_pre(node.clone());
  //   self.travel_child_node(node.clone());
  //   self.travel_node_in_post(node.clone());
  // }

  // pub fn travel_node_in_pre(&mut self, node: Rc<RefCell<&mut Node>>) {
  //   for transform in self.transforms.iter_mut() {
  //     node.borrow_mut().accept_pre_transform(transform, &mut self.ctx);
  //   }
  // }

  // pub fn travel_node_in_post(&mut self, node: Rc<RefCell<&mut Node>>) {
  //   for transform in self.transforms.iter_mut() {
  //     node.borrow_mut().accept_post_transform(transform, &mut self.ctx);
  //   }
  // }

  pub fn travel_node(&mut self, node: Rc<RefCell<Node>>) {
    for transform in self.transforms.iter_mut() {
      node.borrow_mut().accept_pre_transform(transform, &mut self.ctx);
    }
    for child in node.borrow_mut().get_children().into_inner().iter_mut() {
      self.ctx.save_parent(node.clone());
      self.travel_node(child.clone());
    }
    for transform in self.transforms.iter_mut() {
      node.borrow_mut().accept_post_transform(transform, &mut self.ctx);
    }
  }

  pub fn save_parent_node(&mut self, node: Rc<RefCell<&'a mut Node>>) {
    self.ctx.save_parent_node_ref(node);
  }
}

pub struct TransformContext<'a> {
  pub file_name: String,
  pub hoist_static: bool,
  pub components: Vec<String>,
  pub in_v_once: bool,
  pub is_ts: bool,
  pub child_index: usize,
  pub current_node: Option<&'a mut Node>,
  pub parent_node_ref: Option<Rc<RefCell<&'a mut Node>>>,
  pub parent: Option<Rc<RefCell<Node>>>,
  pub directive_transform: Option<HashMap<String, Box<dyn DirectiveTransform>>>,
}

impl<'a> TransformContext<'a> {
  pub fn new(file_name: String) -> Self {
    Self {
      file_name,
      components: vec![],
      hoist_static: false,
      in_v_once: false,
      is_ts: false,
      child_index: 0,
      current_node: None,
      parent_node_ref: None,
      parent: None,
      directive_transform: None,
    }
  }

  pub fn save_parent_node_ref(&mut self, node: Rc<RefCell<&'a mut Node>>) {
    self.parent_node_ref = Some(node);
  }

  pub fn save_parent(&mut self, node: Rc<RefCell<Node>>) {
    self.parent = Some(node);
  }
}

pub trait DirectiveTransform {
  fn transform(
    &self, dir: 
    &mut DirectiveProp, 
    node: &mut Node, ctx: 
    &mut TransformContext) -> DirectiveTransformRes;
}

pub struct DirectiveTransformRes {
  pub properties: Vec<Property>,
  pub need_runtime: bool,
}
