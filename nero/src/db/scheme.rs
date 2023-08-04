use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Scheme {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Scheme {
    pub fn new<T: ToString>(name: T, fields: Vec<Field>) -> Self {
        Self {
            name: name.to_string(),
            fields,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub _type: FieldType,
    pub args: Vec<FieldArg>,
}

impl Field {
    pub fn new<T: ToString>(name: T, _type: FieldType, args: Vec<FieldArg>) -> Self {
        Self {
            name: name.to_string(),
            _type,
            args,
        }
    }
}

#[derive(Serialize, Clone)]
pub enum FieldType {
    Int,
    String,
    Bool,
    LinkTo,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FieldArg {
    MaxLength(usize),
    Default(String),
}
