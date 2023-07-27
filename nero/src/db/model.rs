use crate::db::fieldargs::{IntArgs, StringArg};

pub trait Model {
    fn model_struct() -> &'static ModelStruct;
}

pub struct ModelStruct {
    pub fields: &'static [Field],
}

pub struct Field {
    pub name: &'static str,
    pub field_type: FieldType,
}

pub enum FieldType {
    Int(IntArgs),
    String(StringArg),
    Bool,
    LinkTo,
}
