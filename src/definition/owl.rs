
use definition::definition::Definition;

const OWL_NAMESPACE: &'static str = "http://www.w3.org/2002/07/owl#";

#[derive(Debug)]
pub struct Owl {
}

impl Definition for Owl {
  fn get_namespace(&self) -> &'static str {
    OWL_NAMESPACE
  }

  fn has_property(&self, _property: &str) -> bool {
    false
  }
}
