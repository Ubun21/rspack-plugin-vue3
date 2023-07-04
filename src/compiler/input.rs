use crate::{Position};
pub struct Input<'a> {
  pub offset: usize,
  pub line: usize,
  pub column: usize,
  pub source: &'a str,
}

impl<'a> Input<'a> {
  pub fn new(source: &'a str) -> Self {
    Self {
      offset: 0,
      line: 1,
      column: 1,
      source,
    }
  }

  pub fn consume(&mut self, n: usize) {
    if n > self.source.len() {
      panic!("consume out of range");
    }
    self.column += n;
    self.offset += n;
    let mut i = 0;
    self.source.chars().for_each(|c| {
      if i == n {
        return;
      }
      if c == '\n' {
        self.line += 1;
      }
      i += 1;
    });
    
    self.source = &self.source[n..];
  }

  pub fn skip_start_space(&mut self) {
     self.source
      .chars()
      .take_while(|c| {
        if c.is_whitespace() {
          self.consume(1);
          true
        } else {
          false
        }
      })
      .for_each(|_| {})
  }

  pub fn peek_chars(&self, n: usize) -> &'a str {
    &self.source[..n]
  }

  pub fn peek_char_at(&self, n: usize) -> char {
    self.source.chars().nth(n).unwrap()
  }

  pub fn peek_char(&self) -> char {
    self.source.chars().nth(0).unwrap()
  }

  pub fn has_next_char(&self) -> bool {
    self.source.len() > 0
  }

  pub fn is_end(&self) -> bool {
    self.source.len() == 0
  }

  pub fn source_len(&self) -> usize {
    self.source.len()
  }

  pub fn get_current_position(&self) -> Position {
    Position {
      offset: self.offset,
      line: self.line,
      column: self.column,
    }
  }
}
