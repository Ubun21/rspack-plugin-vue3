use rspack_plugin_vue3::{read_file_sync};
use std::path::Path;
#[test]
pub fn test_input() {
  let content = read_file_sync(
    &Path::new("./tests/transforms/fixtures/a.js"));
  println!("{}", content);
}