use definition::definition::Definition;

const RDF_NAMESPACE: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

#[derive(Debug)]
pub struct Rdf {}

impl Definition for Rdf {
    fn get_namespace(&self) -> &'static str {
        RDF_NAMESPACE
    }

    fn has_property(&self, _property: &str) -> bool {
        false
    }
}
