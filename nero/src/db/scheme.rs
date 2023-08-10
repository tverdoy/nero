use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Scheme {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Field {
    pub name: String,
    pub tp: String,
    pub is_option: bool,
    pub attrs: Attributes,
}

#[derive(Serialize, Debug, Clone)]
pub struct Attributes {
    pub max_length: Option<usize>,
    pub default: Option<String>,
}
