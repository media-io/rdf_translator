use definition::definition::Definition;

const FRANCETV_NAMESPACE: &str = "http://ressources.idfrancetv.fr/ontologies#";

#[derive(Debug)]
pub struct FranceTv {}

impl Definition for FranceTv {
    fn get_namespace(&self) -> &str {
        FRANCETV_NAMESPACE
    }

    fn has_property(&self, _property: &str) -> bool {
        false
    }
}
