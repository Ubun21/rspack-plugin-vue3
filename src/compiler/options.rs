pub struct ParseOptions {
  pub is_void_tag: fn(&str) -> bool,
  pub is_in_pre: fn(&str) -> bool,
  pub comment: bool,
  pub is_native_tag: fn(&str) -> bool,
  pub is_custom_element: fn(&str) -> bool,
  pub is_builtin_component: fn(&str) -> bool,
  pub delimiters: (String, String),
}

impl Default for ParseOptions {
  fn default() -> Self {
      Self { 
        is_void_tag: |_: &str| false,
        is_in_pre: |_: &str| false,
        is_native_tag: |_: &str| true,
        is_custom_element: |_: &str| false,
        is_builtin_component: |_: &str| false,
        delimiters: (String::from("{{"), String::from("}}")),
        comment: true
       }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_default() {
    let options = ParseOptions::default();
    assert_eq!((options.is_void_tag)("br"), false);
  }

  #[test]
  fn test_override_default() {
    let options = ParseOptions {
      is_void_tag: |tag: &str| tag == "br",
      ..Default::default()
    };
    assert_eq!((options.is_void_tag)("br"), true);
  }

  #[test]
  pub fn test_interpolation() {
    let str = String::from("{{hello}}   ");
    let close_index = str.find("}}").unwrap();
    let content = &str[2..close_index];

    println!("{}", content);
  }
}