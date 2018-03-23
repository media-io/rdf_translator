
use jsonpath::Selector;
use serde_json::Value;

pub fn get_one(doc: &Value, path: &str) -> Option<Value> {
  let selector = Selector::new(path).unwrap();

  let results: Vec<&Value> = selector.find(&doc).collect();

  if results.len() != 1 {
    return None;
  }
  if results[0].is_null() {
    return None;
  }
  Some(results[0].clone())
}
