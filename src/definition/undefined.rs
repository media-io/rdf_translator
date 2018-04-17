use definition::definition::Definition;

const UNDEFINED_NAMESPACE: &str = "";

#[derive(Debug)]
pub struct Undefined {}

impl Definition for Undefined {
    fn get_namespace(&self) -> &str {
        UNDEFINED_NAMESPACE
    }

    fn has_property(&self, _property: &str) -> bool {
        false
    }
}
