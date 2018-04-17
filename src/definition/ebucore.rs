use definition::definition::Definition;

const EBUCORE_NAMESPACE: &str = "http://www.ebu.ch/metadata/ontologies/ebucore/ebucore#";

#[derive(Debug)]
pub struct EbuCore {}

impl Definition for EbuCore {
    fn get_namespace(&self) -> &str {
        EBUCORE_NAMESPACE
    }

    fn has_property(&self, property: &str) -> bool {
        match property {
            "businessObjectId" | "title" | "alternativeTitle" | "originalTitle" | "description"
            | "dateModified" | "dateCreated" => true,
            _ => false,
        }
    }
}
