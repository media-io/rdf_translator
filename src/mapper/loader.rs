use converter::converter::Converter;
use jsonpath::Selector;
use rdf::node::Node;
use regex::Regex;
use serde_json;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct Predicate {
    pub namespace: Option<String>,
    pub label: String,
    pub condition: Option<String>,
    pub identifier: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub elements: Option<String>,
    pub predicate: Predicate,
    pub label: Option<String>,
    #[serde(default)] pub language: String,
    #[serde(default)] pub datatype: String,
    #[serde(default)] pub as_uri: bool,
    #[serde(default)] pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    pub elements: Option<String>,
    pub label: String,
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mapper {
    pub objects: Vec<Object>,
}

pub fn parse_label(label: &str, document: &Value) -> Vec<String> {
    let pattern_text = r"([a-zA-Z0-9#$.\\/:?&_-]*)*";
    let pattern_json_path = r"(\{\{[a-zA-Z$@.-_'/()= \*]*\}\})*";

    let mut expression = pattern_text.to_string();
    expression += pattern_json_path;
    expression += pattern_text;
    expression += pattern_json_path;
    expression += pattern_text;
    expression += pattern_json_path;
    expression += pattern_text;

    let regex = Regex::new(&expression).unwrap();

    match regex.captures(label) {
        Some(captures) => {
            let mut results = vec!["".to_string()];

            for (index, capture) in captures.iter().enumerate() {
                if capture.is_none() || index == 0 {
                    continue;
                }

                let m = capture.unwrap();
                let c = m.as_str();

                if !(c.starts_with("{{") && c.ends_with("}}")) {
                    for mut r in &mut results {
                        *r += c;
                    }
                    continue;
                }

                let mut json_path = c.to_string();
                json_path.remove(0);
                json_path.remove(0);
                let len = json_path.len();
                json_path.truncate(len - 2);

                let selector = Selector::new(&json_path).unwrap();
                let founds: Vec<&Value> = selector.find(document).collect();

                let mut tmp_results = vec![];

                for mut r in &mut results {
                    let template = r.clone();
                    for found in &founds {
                        match *(*found) {
                            Value::String(ref content) => {
                                tmp_results.push(template.clone() + content);
                            }
                            Value::Number(ref content) => {
                                tmp_results.push(
                                    template.clone() + &content.as_f64().unwrap().to_string(),
                                );
                            }
                            _ => {}
                        }
                    }
                }

                results = tmp_results;
            }

            results
        }
        None => vec![label.to_string()],
    }
}

impl Mapper {
    pub fn load(filename: &str) -> Mapper {
        let mut content = String::new();
        let mut mapping_file = File::open(filename).unwrap();
        let _ = mapping_file.read_to_string(&mut content).unwrap();
        let mapping: Mapper = serde_json::from_str(&content).unwrap();

        mapping
    }

    pub fn process(&self, document: &Value, converter: &mut Converter) {
        for object in &self.objects {
            if let Some(ref elements_path) = object.elements {
                let selector = Selector::new(&elements_path).unwrap();
                let elements: Vec<&Value> = selector.find(document).collect();

                for element in elements {
                    for subject_label in parse_label(&object.label, &element) {
                        let subject = converter.create_subject(subject_label.as_str());
                        self.process_relations(&element, converter, &subject, &object.items)
                    }
                }
            } else {
                for subject_label in parse_label(&object.label, &document) {
                    let subject = converter.create_subject(subject_label.as_str());
                    self.process_relations(&document, converter, &subject, &object.items)
                }
            }
        }
    }

    fn process_item(
        &self,
        document: &Value,
        converter: &mut Converter,
        subject: &Node,
        item: &Item
    ) {
        let label = item.label.to_owned();
        if let Some(ref condition) = item.predicate.condition {
            let selector = Selector::new(&condition).unwrap();
            let founds: Vec<&Value> =
                selector
                .find(document)
                .filter(|x| !x.is_null())
                .collect();

            if founds.is_empty() {
                return;
            }
        }

        if label.is_none() {
            let child_subject =
                if let Some(ref identifier) = item.predicate.identifier {
                    if let Some(id) = parse_label(identifier, document).first() {
                        converter.create_blank_node_with_id(id)
                    } else {
                        converter.create_blank_node()
                    }
                } else {
                    converter.create_blank_node()
                };

            converter.add_blank(
                subject,
                &item.predicate.namespace,
                &item.predicate.label,
                child_subject.clone(),
            );
            self.process_relations(document, converter, &child_subject, &item.items);
            // continue;
            return;
        }

        let object_labels = parse_label(&label.unwrap(), document);
        if item.predicate.namespace == Some("francetv".to_string()){
            println!("{:?}:{}", item.predicate.namespace, item.predicate.label);
            println!("{:?}", object_labels);
        }
        let object_label = object_labels.first();

        if object_label.is_none() {
            // continue;
            return;
        }

        let content = object_label.unwrap();
        match (item.language.as_str(), item.datatype.as_str(), item.as_uri) {
            ("", "", false) => converter.add(
                subject,
                &item.predicate.namespace,
                &item.predicate.label,
                content,
            ),
            (lang, "", false) => converter.add_with_language(
                subject,
                &item.predicate.namespace,
                &item.predicate.label,
                content,
                lang,
            ),
            ("", datatype, false) => converter.add_with_datatype(
                subject,
                &item.predicate.namespace,
                &item.predicate.label,
                content,
                datatype,
            ),

            ("", "", true) => converter.add_uri(
                subject,
                &item.predicate.namespace,
                &item.predicate.label,
                content,
            ),
            _ => {}
        }
    }

    fn process_relations(
        &self,
        document: &Value,
        converter: &mut Converter,
        subject: &Node,
        items: &[Item],
    ) {
        for item in items.iter() {
            if let Some(ref elements_path) = item.elements {
                let selector = Selector::new(&elements_path).unwrap();
                let elements: Vec<&Value> = selector.find(document).collect();
                for element in elements {
                    self.process_item(element, converter, subject, item);
                }
            } else {
                self.process_item(document, converter, subject, item);
            }
        }
    }
}
