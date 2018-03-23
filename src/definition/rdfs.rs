
use definition::definition::Definition;

const RDFS_NAMESPACE: &'static str = "http://www.w3.org/2000/01/rdf-schema#";

#[derive(Debug)]
pub struct Rdfs {
}

impl Definition for Rdfs {
  fn get_namespace(&self) -> &'static str {
    RDFS_NAMESPACE
  }

  fn has_property(&self, _property: &str) -> bool {
    false
  }
}
