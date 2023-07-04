use std::{collections::HashSet, rc::Rc, cell::RefCell};
use crate::{ 
  Input, 
  SourceLocation, 
  Prop, 
  ErrorCodes, 
  ERROR_MESSAGES, 
  Position,
  ParseOptions,
  NormalProp,
  DirectiveProp, 
  TextPropValue, 
  ConstantTypes, 
  ElementNodeBase, 
  ElementTagType, 
  TextNode, 
  InterpolationNode, 
  SimpleExpressionNode, 
  ExpressionNode, 
  CommentNode, RootNode, Node
};
use regex::Regex;

pub enum TextMode {
  Data,
  RCDATA,
  RAWTEXT,
  CDATA,
}

pub struct Parser<'a> {
  pub input: Input<'a>,
  pub ancestors: Vec<Node>,
  pub mode: TextMode,
  pub parse_options: ParseOptions,
  pub context: Context,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TagType {
  StartTag,
  EndTag,
}

#[derive(Default)]
pub struct Context {
  pub in_pre: bool,
  pub in_v_pre: bool
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            input: Input::new(source),
            ancestors: vec![],
            mode: TextMode::Data,
            parse_options: ParseOptions::default(),
            context: Default::default()
        }
    }

    pub fn new_with_options(source: &'a str, options: ParseOptions) -> Self {
      Self {
        input: Input::new(source),
        ancestors: vec![],
        mode: TextMode::Data,
        parse_options: options,
        context: Default::default()
      }
    }

    pub fn parse(&mut self) -> Node {
      let root = RootNode {
        children: Rc::new(RefCell::new(self.parse_children())),
        helpers: vec![],
        components: vec![],
        directives: vec![],
        hoists: vec![],
        imports: vec![],
        cached: 0,
        temps: 0,
        ssr_helpers: vec![],
        code_gen_node: None,
        filters: vec![],
        loc: SourceLocation {
          start: Position::default(),
          end: Position::default(),
        },
      };
      Node::Root(root)
    }

    pub fn parse_children(&mut self) -> Vec<Rc<RefCell<Node>>> {

      let mut nodes: Vec<Node> = vec![];
      
      while !self.is_end() {
        let mut node: Option<Node> = None;
        match self.mode {
              TextMode::Data => {
                // https://html.spec.whatwg.org/multipage/parsing.html#tag-open-state
                if self.input.peek_char_at(0) == '<' {
                  if self.input.source_len() == 1 {
                    todo!("emit error")
                  } else if self.input.peek_char_at(1) == '!' {
                    node = Some(Node::Comment(self.parse_comment()));
                  } else if self.input.peek_char_at(1) == '/' {
                    if self.input.source_len() == 2 {
                      emit_error(ErrorCodes::EofBeforeTagName);
                    } else if self.input.peek_char_at(2) == '>' {
                      todo!("emit error")
                    } else if self.input.peek_char_at(2).is_ascii_alphabetic() {
                      todo!("emit error")
                    } else {
                      todo!("emit error")
                    }
                    todo!("emit error")
                  } else if self.input.peek_char_at(1).is_ascii_alphabetic() {
                    let el = self.parse_element();
                    node = Some(Node::ElementNode(el));
                  } else {
                    todo!("emit error")
                  }
                } else if start_with(self.input.source, 
                  self.parse_options.delimiters.0.as_str()) {
                  node = Some(Node::Interpolation(self.parse_interpolation()));
                }
              },
              _ => todo!("parse"),
        }

        if node.is_none() {
          node = Some(Node::Text(self.parse_text()));
        }

        nodes.push(node.unwrap());
      }

      nodes.into_iter()
        .filter(|node| {
          match node {
            Node::Comment(_) => self.parse_options.comment,
            _ => true,
          }
        })
        .map(|n| Rc::new(RefCell::new(n)))
        .collect::<Vec<_>>()
    }

    pub fn parse_element(&mut self) -> ElementNodeBase {
      let start_pos = self.input.get_current_position();
      let mut element = self.parse_tag(TagType::StartTag).unwrap();

      if element.is_self_closing || (self.parse_options.is_void_tag)(&element.tag_name) {
        let end_pos = self.input.get_current_position();
        element.loc = SourceLocation {
          start: start_pos,
          end: end_pos,
        };
        return element;
      }

      self.ancestors.push(Node::ElementNode(element));
      let mut children = self.parse_children();
      let parent = self.last_ancestor_mut().unwrap();
      // 
      parent.children.clone().borrow_mut().append(&mut children);
      let mut element = match self.ancestors.pop().unwrap() {
        Node::ElementNode(element) => element,
        _ => todo!("only element node")
      };

      if start_with_end_tag_open(self.input.source, &element.tag_name) {
        self.parse_tag(TagType::EndTag);
      } else {
        emit_error(ErrorCodes::XMissingEndTag)
      }
      let end_pos = self.input.get_current_position();
      element.loc = SourceLocation {
        start: start_pos,
        end: end_pos,
      };
      element
    }

    pub fn parse_tag(&mut self, tag_type: TagType) -> Option<ElementNodeBase> {
      let reg = Regex::new(r"^</?([a-zA-Z][^\t\r\n\f />]*)").unwrap();
      let result = reg.captures(self.input.source);
      if result.is_none() {
        todo!("emit error")
      }
      let captures = &result.unwrap();
      let tag_name = captures.get(1).unwrap().as_str();
      let len = captures.get(0).unwrap().as_str().len();
      self.input.consume(len);
      self.input.skip_start_space();

      if (self.parse_options.is_in_pre)(tag_name) {
        self.context.in_pre = true;
      }

      let attributes = self.parse_attributes(tag_type.clone());

      let mut is_self_closing = false;
      if self.input.source_len() == 0 {
        emit_error(ErrorCodes::EofInTag);
      } else {
        is_self_closing = start_with(self.input.source, "/>");
        self.input.consume(if is_self_closing { 2 } else { 1 });
      }

      if tag_type == TagType::EndTag {
        return None;
      }

      let mut element_type = ElementTagType::PlainElementNode;
      if !self.context.in_v_pre {
        match tag_name {
          "slot" => {
            element_type = ElementTagType::SlotOutletNode;
          },
          "template" => {
            if attributes.iter().any(|attr| {
              match attr {
                Prop::Directive(_) => true,
                _ => false,
              }
            }) {
              element_type = ElementTagType::TemplateNode;
            }
          },
          _ => {
            if self.is_component(tag_name, &attributes) {
              element_type = ElementTagType::ComponentNode;
            }
          }
        }

      }
      
      Some(
        ElementNodeBase { 
          tag_type: element_type, 
          tag_name: tag_name.to_string(),
          is_self_closing,
          props: Rc::new(RefCell::new(attributes)),
          children: Rc::new(RefCell::new(vec![])), 
          loc: SourceLocation {
            start: Position::default(),
            end: Position::default(),
          },
          code_gen: None,
        }
      )
    }

    pub fn parse_text(&mut self) -> TextNode {
      let start_pos = self.input.get_current_position();
      let mut content = String::new();
      while !self.input.is_end() {
        if self.input.peek_char_at(0) == '<' {
          break;
        }
        content.push(self.input.peek_char());
        self.input.consume(1);
      }
     TextNode {
        content,
        loc: SourceLocation {
          start: start_pos,
          end: self.input.get_current_position(),
        },
      }
    }

    pub fn parse_interpolation(&mut self) -> InterpolationNode {
      let (open, close) = self.parse_options.delimiters.clone();
      let start_pos = self.input.get_current_position();
      self.input.consume(open.len());
      let close_index = self.input.source.find(close.as_str());
      if close_index.is_none() {
        emit_error(ErrorCodes::XMissingInterpolationEnd);
      }
      let close_index = close_index.unwrap();
      let content = self.input.source[..close_index].to_string();
      self.input.consume(content.len() + close.len());
      InterpolationNode {
        content: ExpressionNode::SimpleExpressionNode(SimpleExpressionNode {
          content,
          is_static: false,
          constant_type: ConstantTypes::NotConstant,
          loc: SourceLocation {
            start: start_pos,
            end: self.input.get_current_position(),
          },
        }),
      }
    }

    pub fn parse_comment(&mut self) -> CommentNode {
      let start_pos = self.input.get_current_position();
      self.input.consume(4);
      let mut content = String::new();
      while self.input.has_next_char() {
        if self.input.peek_chars(3) == "-->" {
          break;
        }
        content.push(self.input.peek_char());
        self.input.consume(1);
      }
      if !start_with(self.input.source, "-->") {
        todo!("emit error not end comment")
      }
      self.input.consume(3);
      CommentNode {
        content,
        loc: SourceLocation {
          start: start_pos,
          end: self.input.get_current_position(),
        },
      }
    }

    pub fn parse_attributes(&mut self, tag_type: TagType) -> Vec<Prop> {
      let mut attributes = vec![];
      let mut name_set = HashSet::new();
      while !self.input.is_end()  {
        self.input.skip_start_space();
        if self.input.peek_char() == '>' || self.input.peek_chars(2) == "/>" {
          break;
        }
        match tag_type {
           TagType::EndTag => {
            emit_error(ErrorCodes::EndTagWithAttributes)
           },
           _ => {} 
        }
        attributes.push(self.parse_attribute(&mut name_set));
      }
      attributes
    }

    pub fn parse_attribute(&mut self, name_set: &mut HashSet<String>) -> Prop {
      let mut start_pos = self.input.get_current_position();
      let reg = Regex::new(r"^([^\t\r\n\f />][^\t\r\n\f />=]*)").unwrap();
      let result = reg.captures(self.input.source);
      if result.is_none() {
        todo!("emit error")
      }
      let captures = &result.unwrap();
      let name = captures.get(0).unwrap().as_str();
      if name_set.contains(name) {
        todo!("duplicate attribute name")
      }
      name_set.insert(name.to_string());

      if name.chars().nth(0).unwrap() == '=' {
        todo!("emit error attribute name can not start with '='")
      }

      self.input.consume(name.len());

      let reg = Regex::new(r"^[\t\r\n\f ]*=").unwrap();
      let value;
      if reg.is_match(self.input.source) {
        self.input.skip_start_space();
        self.input.consume(1); // consume '='
        self.input.skip_start_space();
        value = self.parse_attribute_value();
      } else {
        value = None;
      }
      let end_pos = self.input.get_current_position();

      let directive_reg = Regex::new(r"^(v-[a-zA-Z0-9-]|:|\.|@|#)").unwrap();
      if !self.context.in_v_pre && directive_reg.is_match(name) {
        let reg = 
          Regex::new(r"(?:^v-([a-z0-9-]+))?(?:(?::|^\.|^@|^#)(\[[^\]]+\]|[^\.]+))?(.+)?$")
          .unwrap();
        let matched = reg.captures(name).unwrap();

        let arg_pos = &mut start_pos;
        let is_prop_short_hand = start_with(name, ".");
        let dir_name = match matched.get(1) {
          Some(name) => {
            arg_pos.offset += name.as_str().len();
            arg_pos.column += name.as_str().len();
            name.as_str()
          },
          None => {
            if is_prop_short_hand || start_with(name, ":") {
              "bind"
            } else if start_with(name, "@") {
              "on"
            } else {
              "slot"
            }
          },
        };

        let arg = match matched.get(2) {
          Some(arg) => {

            let mut content = arg.as_str();
            let mut is_static = true;
            if content.starts_with("[") {
              is_static = false;
              if !content.ends_with("]") {
                emit_error(ErrorCodes::XMissingDynamicDirectiveArgumentEnd);
              }
              content = content.trim_start_matches("[").trim_end_matches("]");
            }

            let arg_end_pos = Position {
              line: arg_pos.line,
              column: arg_pos.column + content.len(),
              offset: arg_pos.offset + content.len(),
            };

            Some(ExpressionNode::SimpleExpressionNode(SimpleExpressionNode {
              content: content.to_string(),
              is_static,
              constant_type: match is_static {
                  true => ConstantTypes::CanStringify,
                  false => ConstantTypes::NotConstant
              },
              loc: SourceLocation {
                start: arg_pos.clone(),
                end: arg_end_pos,
              },
            }))
          },
          None => None,
        };

        let modifiers = match matched.get(3) {
          Some(modifiers) => {
            modifiers.as_str().trim_start_matches(".")
              .split(".").map(|m| m.to_string()).collect()
          },
          None => vec![],
        };

        let exp = match value {
            Some(attribute) => {
              let exp_start_pos = attribute.loc.start;
              let exp_end_pos = attribute.loc.end;
              Some(ExpressionNode::SimpleExpressionNode(SimpleExpressionNode {
                content: attribute.content,
                is_static: false,
                constant_type: ConstantTypes::NotConstant,
                loc: SourceLocation {
                  start: exp_start_pos,
                  end: exp_end_pos,
                }
              }))
            },
            None => None,
        };

        return Prop::Directive(DirectiveProp {
          name: dir_name.to_string(),
          arg,
          exp,
          modifiers,
          loc: SourceLocation {
            start: start_pos,
            end: end_pos,
          },
        });
      }

      Prop::Normal(NormalProp {
        name: name.to_string(),
        value,
        loc: SourceLocation {
          start: start_pos,
          end: end_pos,
        },
      })
    }

    pub fn parse_attribute_value(&mut self) -> Option<TextPropValue> {
      let quote = self.input.peek_char();
      let mut attribute_value = TextPropValue {
        content: "".to_string(),
        loc: SourceLocation {
          start: Position::default(),
          end: Position::default(),
        },
      };
      let is_quoted = quote == '"' || quote == '\'';
      if is_quoted {

        let start_pos = self.input.get_current_position();
        attribute_value.loc.start = start_pos;
        self.input.consume(1);

        let end_index = self.input.source.find(quote);
        if end_index.is_none() {
          todo!("emit error")
        }

        self.parse_text_data(end_index.unwrap(), &mut attribute_value);
        self.input.consume(1);
        let end_pos = self.input.get_current_position();
        attribute_value.loc.end = end_pos;
        return Some(attribute_value);
      } else {
        let reg = Regex::new(r"^[^\t\r\n\f >]+").unwrap();
        let matched = reg.captures(self.input.source);
        if matched.is_none() {
          return None;
        }

        let match_str = match matched {
          Some(m) => {
            match m.get(0) {
              Some(str) => str.as_str(),
              _ => "",
            }
          },
          None => "",
        };

        let unexpected_char = Regex::new(r#"['"<=`]"#).unwrap();
        for _ in unexpected_char.find_iter(match_str) {
          emit_error(ErrorCodes::UnexpectedCharacterInUnquotedAttributeValue);
        }

        let start_pos = self.input.get_current_position();
        attribute_value.loc.start = start_pos;
        let size = match_str.len();
        self.parse_text_data(size, &mut attribute_value);
        let end_pos = self.input.get_current_position();
        attribute_value.loc.end = end_pos;
        return Some(attribute_value);
      }
    }

    pub fn parse_text_data(&mut self, index: usize, value: &mut TextPropValue) {
      let raw_text = self.input.source[..index].to_string();
      value.content.push_str(raw_text.as_str());
      self.input.consume(index);
    }

    pub fn is_end(&self) -> bool {
      match self.mode {
        TextMode::Data => {
          if self.input.is_end() {
            return true;
          }
          if self.input.peek_chars(2) == "</" {
            let top = self.last_ancestor();
            match top {
              Some(el) => {
                if self.input.peek_chars(el.tag_name.len() + 3) == format!("</{}>", el.tag_name) {
                  return true;
                }
              }
              _ => { 
                return false;
              }
            }
          }
          false
        },
        _ => self.input.is_end(),
      }
    }

    pub fn last_ancestor(&self) -> Option<&ElementNodeBase> {
      match self.ancestors.last() {
        Some(Node::ElementNode(el)) => Some(el),
        _ => None,
      }
    }

    pub fn last_ancestor_mut(&mut self) -> Option<&mut ElementNodeBase> {
      match self.ancestors.last_mut() {
        Some(Node::ElementNode(el)) => Some(el),
        _ => None,
      }
    }

    pub fn is_component(&self, tag: &str, attributes: &Vec<Prop>) -> bool {
      if (self.parse_options.is_custom_element)(tag) {
        return false;
      }
      let alpha = Regex::new(r"^[A-Z]").unwrap();
      if tag == "component" ||
         alpha.is_match(tag) ||
         is_core_component(tag) ||
         (self.parse_options.is_builtin_component)(tag) ||
         !(self.parse_options.is_native_tag)(tag) {
        return true;
      }
      if attributes.iter().any(|attr| {
        match attr {
          Prop::Directive(directive) => {
            directive.name == "is"
          },
          _ => false,
        }
      }) {
        return true;
      }
      false
    }
}

pub fn is_core_component(tag: &str) -> bool {
  tag == "Teleport" || 
  tag == "Suspense" || 
  tag == "KeepAlive" || 
  tag == "BaseTransition"
}

pub fn str_is_equal(str1: &str, str2: &str) -> bool {
    str1.eq_ignore_ascii_case(str2)
}

pub fn start_with_end_tag_open(source: &str, tag: &str) -> bool {
    source.starts_with("</") && 
    source[2..].starts_with(tag) 
}

pub fn start_with(source: &str, prefix: &str) -> bool {
    source.starts_with(prefix)
}

pub fn emit_error(code: ErrorCodes) {
  println!("emit error: {:?}", ERROR_MESSAGES.get(&code));
  todo!("emit error with")
}

#[cfg(test)]
mod tests {
  #[test]
  pub fn test_run() {
    assert_eq!(1, 1)
  }
}
