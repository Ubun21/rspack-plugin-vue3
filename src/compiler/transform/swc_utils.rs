use swc_common::{
  self,
  errors::{ColorConfig, Handler},
  sync::Lrc,
  FileName, SourceMap
};
use swc_ecma_ast::{Module, Ident, Expr, AssignExpr, ObjectLit, PropOrSpread, Prop, PropName, Function, MemberExpr, MemberProp};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax};
use swc_ecma_visit::{VisitMut, VisitMutWith};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};


pub struct ProcessIdentifiers<'a> {
  pub source: &'a mut String,
  pub cm: Option<Lrc<SourceMap>>,
  pub module: Option<Module>
}

impl<'a> ProcessIdentifiers<'a> {
  pub fn new(source: &'a mut String) -> Self {
    let cm: Lrc<SourceMap> = Default::default();
    Self {
      source,
      cm: Some(cm),
      module: None
    }
  }
  pub fn parse(&mut self) {
    if self.cm.is_none() {
      return;
    }
    let cm = self.cm.clone().unwrap();
    let handler = Handler::with_tty_emitter(
      ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm.new_source_file(
        FileName::Custom("test.js".into()),
        self.source.clone(),
    );

    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let capturing = Capturing::new(lexer);

    let mut parser = Parser::new_from(capturing);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let module = parser
        .parse_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("Failed to parse module.");
    self.module = Some(module);
  }

  pub fn rewrite_identifiers(&mut self) {
    if self.cm.is_none() {
      return;
    }

    if let Some(m) = &mut self.module {
      let mut rewriter = RewriteAstNode;
      m.visit_mut_with(&mut rewriter);
    }
  }

  pub fn generate(&self) -> String {
    if self.cm.is_none() {
      return "".to_string();
    }

    let cm = self.cm.clone().unwrap();

    let mut code = String::new();
    if let Some(m) = &self.module {
      let mut buf = vec![];

      {
          let mut emitter = Emitter {
              cfg: Default::default(),
              cm: cm.clone(),
              comments: None,
              wr: JsWriter::new(cm, "\n", &mut buf, None),
          };

          emitter.emit_module(&m).unwrap();
      }

      code = String::from_utf8_lossy(&buf).to_string()
    }

    code
  }
}

pub struct RewriteAstNode;

impl VisitMut for RewriteAstNode {
  fn visit_mut_assign_expr(&mut self, n: &mut AssignExpr) { 
    let mut visitor: RewriteIdentifier<fn(&Ident) -> bool> = RewriteIdentifier { 
      is_need_rewrite: None
    }; 

    n.visit_mut_with(&mut visitor);
  }

  fn visit_mut_function(&mut self, n: &mut Function) { 
    let mut param_names = CollectIdentifiersName::new();
    n.params.visit_mut_children_with(&mut param_names);

    let mut visitor: RewriteIdentifier<_> = RewriteIdentifier { 
      is_need_rewrite: Some(|i: &Ident| {
        let params = &param_names;
        !params.names.contains(&i.sym.to_string())
      })
    };

    n.body.visit_mut_children_with(&mut visitor);
  }

  fn visit_mut_member_expr(&mut self, n: &mut MemberExpr) { 
    let mut rewrite_visitor: RewriteIdentifier<fn(&Ident) -> bool> = RewriteIdentifier {
      is_need_rewrite: None
    };

    match &mut *n.obj {
      Expr::Ident(i) => {
        i.visit_mut_with(&mut rewrite_visitor);
      },
      Expr::Member(m) => {
        self.visit_mut_member_expr(m);
      },
      _ => {}
    }

    match &mut n.prop {
      MemberProp::Computed(c) => {
        match &mut *c.expr {
          Expr::Ident(i) => {
            i.visit_mut_with(&mut rewrite_visitor);
          },
          _ => {}
        }
      },
      _ => {}
    }
  }

  fn visit_mut_object_lit(&mut self, n: &mut ObjectLit) {
    for p in n.props.iter_mut() {
      if let PropOrSpread::Prop(prop) = p {
        match prop.as_mut() {
          Prop::Shorthand(i) => {
            *prop = Box::new(Prop::KeyValue(swc_ecma_ast::KeyValueProp {
              key: PropName::Ident(i.clone()),
              value: Box::new(Expr::Ident(Ident::new(format!("_ctx.{}", i.sym.to_string()).into(), i.span.clone())))
            }))
          },
          Prop::KeyValue(kv) => {
            match &mut *kv.value {
              Expr::Ident(i) => {
                let mut rewrite_visitor: RewriteIdentifier<fn(&Ident) -> bool> = RewriteIdentifier {
                  is_need_rewrite: None
                };
                i.visit_mut_with(&mut rewrite_visitor);
              },
              Expr::Fn(fn_exp) => {
                self.visit_mut_function(&mut fn_exp.function);
              },
              _ => {}
            }
          },
          Prop::Method(m) => {
            self.visit_mut_function(&mut m.function);
          }
          _ => {}
        }
      }
    }
  }
}

pub struct RewriteIdentifier<F: FnMut(&Ident) -> bool>{
  pub is_need_rewrite: Option<F>
}

impl<F: FnMut(&Ident) -> bool> VisitMut for RewriteIdentifier<F> {
  fn visit_mut_ident(&mut self, n: &mut Ident) { 
    if let Some(f) = &mut self.is_need_rewrite {
      if f(n) {
        n.sym = format!("_ctx.{}", n.sym.to_string()).into();
      }
      return;
    }
    n.sym = format!("_ctx.{}", n.sym.to_string()).into();
  }
}

pub struct CollectIdentifiersName {
  pub names: Vec<String>
}

impl CollectIdentifiersName {
  pub fn new() -> Self {
    Self {
      names: vec![]
    }
  }
}

impl VisitMut for CollectIdentifiersName {
  fn visit_mut_ident(&mut self, n: &mut Ident) { 
    self.names.push(n.sym.to_string());
  }
}
