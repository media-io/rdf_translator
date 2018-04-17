
pub trait Definition {
    fn get_namespace(&self) -> &str;

    fn has_property(&self, property: &str) -> bool;

    fn get_label(&self, l: &str) -> String {
        let mut field = String::from(Self::get_namespace(self));
        field.push_str(l);
        field
    }
}
