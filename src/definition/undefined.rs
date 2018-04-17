use definition::definition::Definition;

const UNDEFINED_NAMESPACE: &'static str = "";

#[derive(Debug)]
pub struct Undefined {}

impl Definition for Undefined {
    fn get_namespace(&self) -> &'static str {
        UNDEFINED_NAMESPACE
    }

    fn has_property(&self, _property: &str) -> bool {
        false
    }
}
