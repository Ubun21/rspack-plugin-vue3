use std::{cell::RefCell, rc::Rc};

use crate::{Transform, TransformContext};

pub enum Namespace {
  /// `http://www.w3.org/1999/xhtml`
  HTML,
  /// `http://www.w3.org/1998/Math/MathML`
  MATHML,
  /// `http://www.w3.org/2000/svg`
  SVG,
  /// `http://www.w3.org/1999/xlink`
  XLINK,
  /// `http://www.w3.org/XML/1998/namespace`
  XML,
  /// `http://www.w3.org/2000/xmlns/`
  XMLNS,
}

#[derive(Debug, Clone, Default, PartialEq, Copy)]
pub struct Position {
  pub offset: usize,
  pub column: usize,
  pub line: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Copy)]
pub struct SourceLocation {
  pub start: Position,
  pub end: Position,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Node {
  Root(RootNode),
  ElementNode(ElementNodeBase),
  Text(TextNode),
  Comment(CommentNode),
  SimpleExpression(SimpleExpressionNode),
  Interpolation(InterpolationNode),
  Attribute(Prop),
  #[default]
  Directive,
  // containers
  CompoundExpression(CompoundExpressionNode),
  If(IfNode),
  IFBranch(IfBranchNode),
  For(ForNode),
  TextCall(TextCallNode),
  // codegen
  VnodeCall(VnodeCall),
  JsCallExpression,
  JsObjectExpression,
  JsProperty,
  JsArrayExpression,
  JsFunctionExpression,
  JsConditionalExpression,
  JsCacheExpression,

  // ssr codegen
  JsBlockStatement,
  JsTemplateLiteral,
  JsIfStatement,
  JsAssignmentExpression,
  JsSequenceExpression,
  JsReturnStatement,

  // 
  Dummy,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextNode {
  pub content: String,
  pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommentNode {
  pub content: String,
  pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RootNode {
  pub children: Rc<RefCell<Vec<Rc<RefCell<Node>>>>>,
  pub helpers: Vec<String>,
  pub components: Vec<String>,
  pub directives: Vec<String>,
  pub hoists: Vec<String>,
  pub imports: Vec<String>,
  pub cached: usize,
  pub temps: usize,
  pub ssr_helpers: Vec<String>,
  pub code_gen_node: Option<VnodeCall>,
  pub filters: Vec<String>,
  pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementNodeBase {
  pub tag_type: ElementTagType,
  pub tag_name: String,
  pub is_self_closing: bool,
  pub props: Rc<RefCell<Vec<Prop>>>,
  pub children: Rc<RefCell<Vec<Rc<RefCell<Node>>>>>,
  pub code_gen: Option<ElementNodeCodeGen>,
  pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementTagType {
  PlainElementNode,
  ComponentNode,
  SlotOutletNode,
  TemplateNode,  
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum CodeGenType {
  #[default]
  PlainElementNodeCodeGen,
  ComponentNodeCodeGen,
  TemplateNodeCodeGen,
  SlotOutletNodeCodeGen,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ElementNodeCodeGen {
  pub gen_type: CodeGenType,
  pub vnode_call: Option<VnodeCall>,
  pub simple_expression_node: Option<SimpleExpressionNode>,
  pub cache_expression: Option<CacheExpression>,
  pub memo_expression: Option<MemoExpression>,
  pub render_slot_call: Option<RenderSlotCall>,
}


#[derive(Debug, Clone, PartialEq, Default)]
pub struct RenderSlotCall {
  pub call: String,
  pub args: Vec<RenderSlotCallArgs>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RenderSlotCallArgs {
  RawText(String),
  SimpleExpressionNode(SimpleExpressionNode),
  CompoundExpressionNode(CompoundExpressionNode),
  ObjectProperty(ObjectExpression),
  TemplateChildNode(TemplateChildNode),
}


#[derive(Debug, Clone, PartialEq)]
pub struct InterpolationNode {
  pub content: ExpressionNode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
  SimpleExpressionNode(SimpleExpressionNode),
  CompoundExpressionNode(CompoundExpressionNode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleExpressionNode {
  pub content: String,
  pub is_static: bool,
  pub constant_type: ConstantTypes,
  pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundExpressionNode {
  pub children: Vec<CompoundExpressionNodeChild>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompoundExpressionNodeChild {
  SimpleExpressionNode(SimpleExpressionNode),
  InterpolationNode(InterpolationNode),
  TextNode(TextNode),
  RawText(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantTypes {
  NotConstant,
  CanSkipPatch,
  CanHoist,
  CanStringify,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfNode {
  pub branches: Vec<IfBranchNode>,
  pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfBranchNode {
  pub condition: Option<ExpressionNode>,
  pub children: Rc<RefCell<Vec<Rc<RefCell<Node>>>>>,
  pub uer_key: Option<Prop>,
  pub is_template_if: bool,
  pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForNode {
  pub source: ExpressionNode,
  pub value_alias: Option<ExpressionNode>,
  pub key_alias: Option<ExpressionNode>,
  pub object_index_alias: Option<ExpressionNode>,
  pub children: Vec<TemplateChildNode>,
  pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextCallNode {
  pub content: TextCallNodeContent,
  pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextCallNodeContent {
  CompoundExpressionNode(CompoundExpressionNode),
  InterpolationNode(InterpolationNode),
  TextNode(TextNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateChildNode {
  ElementNode(ElementNodeBase),
  TextNode(TextNode),
  CommentNode(CommentNode),
  InterpolationNode(InterpolationNode),
  CompoundExpressionNode(CompoundExpressionNode),
  IfNode(IfNode),
  ForNode(ForNode),
  TextCallNode(TextCallNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateTextChildNode {
  TextNode(TextNode),
  InterpolationNode(InterpolationNode),
  CompoundExpressionNode(CompoundExpressionNode),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VnodeCall {
  pub tag: String,
  pub props: Option<PropExpression>,
  pub patch_flag: Option<String>,
  pub dynamic_props: Option<DynamicProps>,
  pub children: Vec<Node>,
  pub directives: String,
  pub is_block: bool,
  pub disable_tracking: bool,
  pub is_component: bool,
  pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DynamicProps {
  RawText(String),
  SimpleExpressionNode(SimpleExpressionNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VnodeCallChild {
  TextNode(TextNode),
  InterpolationNode(InterpolationNode),
  CompoundExpressionNode(CompoundExpressionNode),
  ElementNode(Box<ElementNodeBase>),
  CommentNode(CommentNode),
  IfNode(IfNode),
  ForNode(ForNode),
  TextCallNode(TextCallNode),
  SimpleExpressionNode(SimpleExpressionNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Prop {
  Normal(NormalProp),
  Directive(DirectiveProp),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NormalProp {
  pub name: String,
  pub value: Option<TextPropValue>,
  pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextPropValue {
  pub content: String,
  pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectiveProp {
  pub name: String,
  pub arg: Option<ExpressionNode>,
  pub exp: Option<ExpressionNode>,
  pub modifiers: Vec<String>,
  pub loc: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropExpression {
  ObjectExpression(ObjectExpression),
  CallExpression(CallExpression),
  SimpleExpression(ExpressionNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsChildNode {
  CallExpression(CallExpression),
  FunctionExpression(FunctionExpression),
  ExpressionNode(ExpressionNode),
  ArrayExpression(ArrayExpression),
  ConditionalExpression(ConditionalExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpression {
  pub callee: String,
  pub arguments: Vec<CallExpressionArgument>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CallExpressionArgument {
  RawText(String),
  JsChildNode(Box<JsChildNode>),
  PropExpression(PropExpression),
  TemplateChildNode(TemplateChildNode),
  TemplateChildNodes(Vec<TemplateChildNode>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectExpression {
  pub properties: Vec<Property>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
  pub key: ExpressionNode,
  pub value: Box<JsChildNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayExpression {
  pub elements: Vec<ElementValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementValue {
  RawText(String),
  ExpressionNode(JsChildNode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpression {
  pub params: Option<Params>,
  pub returns: Option<Returns>,
  pub body: Option<Body>,
  pub newline: bool,
  pub is_slot: bool,
  pub is_non_scoped_slot: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Params {
  RawText(String),
  ExpressionNode(ExpressionNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Returns {
  TemplateChildNode(TemplateChildNode),
  TemplateChildNodes(Vec<TemplateChildNode>),
  JsChildNode(Box<JsChildNode>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Body {
  BlockStatement,
  IfStatement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalExpression {
  pub test: Box<JsChildNode>,
  pub consequent: Box<JsChildNode>,
  pub alternate: Box<JsChildNode>,
  pub newline: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CacheExpression {
  pub index: usize,
  pub value: Box<JsChildNode>,
  pub is_vnode: bool,
  pub newline: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemoExpression {
  pub index: usize,
  pub value: Box<JsChildNode>,
  pub is_vnode: bool,
  pub newline: bool,
}

pub struct TreeNode {
  pub children: Vec<Box<TreeNode>>,
  pub parent: Option<Box<TreeNode>>,
  pub code_gen: CodeGens,
}

pub struct CodeGens {
  pub children: Vec<Box<CodeGens>>,
}

impl Node {
  pub fn accept_pre_transform(&mut self, transform: &mut Box<dyn Transform>, ctx: &mut TransformContext) {
    transform.pre_transform(self, ctx);
  }

  pub fn accept_post_transform(&mut self, transform: &mut Box<dyn Transform>, ctx: &mut TransformContext) {
    transform.post_transform(self, ctx);
  }

  pub fn get_children(&mut self) -> Rc<RefCell<Vec<Rc<RefCell<Node>>>>> {
    match self {
      Node::Root(root_node) => root_node.children.clone(),
      Node::ElementNode(element_node) => element_node.children.clone(),
      _ => panic!("get_children"),
    }
  }

  pub fn create_if_branch_by_element_base(&mut self, d: DirectiveProp) -> IfBranchNode {
    match self {
      Node::ElementNode(el) => {
        let if_branch = create_if_branch(el, d);
        if_branch
      },
      _ => panic!("create_if_branch_by_element_base"),
    }
  }
}

pub fn create_if_branch(
  node: &mut ElementNodeBase, 
  dir: DirectiveProp
) -> IfBranchNode {
  let condition = if dir.name == "else" {
    None
  } else {
    dir.exp
  };
  IfBranchNode { 
    condition, 
    children: node.children.clone(),
    uer_key: find_prop(node.clone(), "key"), 
    is_template_if: false, 
    loc: Default::default()
  }
}

pub fn find_prop(
  node: ElementNodeBase,
  name: &str
) -> Option<Prop> {
  for p in node.props.clone().into_inner().iter() {
    match p {
      Prop::Directive(d) => {
        if d.name == name {
          return Some(Prop::Directive(d.clone()));
        }
      },
      Prop::Normal(a) => {
        if a.name == name {
          return Some(Prop::Normal(a.clone()));
        }
      }
    }
  }
  None
}
