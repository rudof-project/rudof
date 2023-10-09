#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ObjectLiteral {
    value: String,
    language: Option<String>,

    #[serde(rename = "type")]
    type_: Option<String>,
}
