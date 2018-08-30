use definition::Definition;

const XSI_NAMESPACE: &str = "http://www.w3.org/2001/XMLSchema-instance";

#[derive(Debug)]
pub struct Xsi {}

impl Definition for Xsi {
    fn get_namespace(&self) -> &str {
        XSI_NAMESPACE
    }

    fn has_property(&self, _property: &str) -> bool {
        false
    }
}
