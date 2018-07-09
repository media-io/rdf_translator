extern crate rdf_translator;
#[macro_use]
extern crate serde_json;

use serde_json::Value;
use rdf_translator::mapper::loader::*;

#[test]
fn test_parse_label_null() {
    let label = "{{$.id}}";
    let document = Value::Null;
    let result = parse_label(label, &document);
    assert_eq!(result.is_empty(), true);

    let label = "{{$.id}}";
    let document = json!({ "id": 666 as usize });
    let result = parse_label(label, &document);
    assert_eq!(result, vec!["666".to_string()]);

    let label = "prefix{{$.id}}suffix";
    let document = json!({"id": 666 });
    let result = parse_label(label, &document);
    assert_eq!(result, vec!["prefix666suffix".to_string()]);

    let label = "prefix{{$.books.*.id}}suffix";
    let document = json!({"books": [{"id": 666}, {"id": 999}]});
    let result = parse_label(label, &document);
    assert_eq!(
        result,
        vec!["prefix666suffix".to_string(), "prefix999suffix".to_string()]
    );

    let label = "prefix{{$.books.*.id}}_{{$.books.*.key}}suffix";
    let document = json!({"books": [{"id": 666, "key": "aaa"}, {"id": 999, "key": "bbb"}]});
    let result = parse_label(label, &document);
    assert_eq!(
        result,
        vec![
            "prefix666_aaasuffix".to_string(),
            "prefix666_bbbsuffix".to_string(),
            "prefix999_aaasuffix".to_string(),
            "prefix999_bbbsuffix".to_string(),
        ]
    );
}
