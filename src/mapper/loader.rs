
use converter::converter::Converter;
use mapper::json::get_one;
use regex::Regex;
use serde_json;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct Predicate {
  pub namespace: String,
  pub label: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
  pub predicate: Predicate,
  pub json_path: String,
  #[serde(default)]
  pub language: String,
  #[serde(default)]
  pub datatype: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
  pub label: String,
  pub items: Vec<Item>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mapper {
  pub objects: Vec<Object>
}

impl Mapper {
  pub fn load() -> Mapper {
    let mut content = String::new();
    let mut mapping_file = File::open("tests/mapping.json").unwrap();
    let _ = mapping_file.read_to_string(&mut content).unwrap();
    let mapping: Mapper = serde_json::from_str(&content).unwrap();

    mapping
  }

  pub fn process(&self, document: Value, converter: &mut Converter) {
    for object in self.objects.iter() {

      let regex = Regex::new(r"([a-zA-Z$.\\/:?&]*)(\{\{[a-zA-Z$.]*\}\})([a-zA-Z$.\\/:?&]*)").unwrap();
      
      let label =
        match regex.captures(&object.label) {
          Some(captures) => {
            let mut result = "".to_string();
            for capture in captures.iter() {
              let m = capture.unwrap();
              if m.start() == 0 && m.end() == object.label.len() {
                continue;
              }

              let c = m.as_str();
              if c.starts_with("{{") && c.ends_with("}}") {
                let mut json_path = c.to_string();
                json_path.remove(0);
                json_path.remove(0);
                let len = json_path.len();
                json_path.truncate(len - 2);

                match get_one(&document, &json_path) {
                  Some(Value::String(content)) => {
                    result += &content;
                  },
                  _ => {},
                }

              } else {
                result += c;
              }
            }

            result
          },
          None => {
            object.label.clone()
          },
        };

      let subject = converter.create_subject(label.as_str());
      for item in object.items.iter() {
        match (get_one(&document, &item.json_path), item.language.as_str(), item.datatype.as_str()) {
          (Some(Value::String(content)), "", "") => {
            converter.add(&subject, &item.predicate.namespace, &item.predicate.label, content)
          },
          (Some(Value::String(content)), lang, "") => {
            converter.add_with_language(&subject, &item.predicate.namespace, &item.predicate.label, content, lang)
          },
          (Some(Value::String(content)), "", datatype) => {
            converter.add_with_datatype(&subject, &item.predicate.namespace, &item.predicate.label, content, datatype)
          },
          _ => {},
        }
      }
    }
  }
}
