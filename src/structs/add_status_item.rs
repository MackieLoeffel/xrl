use ViewId;

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct AddStatusItem {
    pub source: String,
    pub key: String,
    pub value: String,
    pub alignment: String,
    pub view_id: ViewId,
}

#[test]
fn deserialize_ok() {
    use serde_json;
    use std::str::FromStr;

    let s = r#"{"source": "synctex","key":"0", "value":"0", "alignment":"0","view_id":"view-id-1"}"#;
    let deserialized: Result<AddStatusItem, _> = serde_json::from_str(s);
    let scroll_to = AddStatusItem {
        source: "synctex".to_string(),
        key: "0".to_string(),
        value: "0".to_string(),
        alignment: "0".to_string(),
        view_id: FromStr::from_str("view-id-1").unwrap(),
    };
    assert_eq!(deserialized.unwrap(), scroll_to);
}
