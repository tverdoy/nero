use nero::db::fieldargs::{IntArgs, StringArg};
use nero::db::model::{Field, FieldType, Model, ModelStruct};
use serde::Serialize;

static STRUCT: &ModelStruct = &ModelStruct {
    fields: &[
        Field {
            name: "id",
            field_type: FieldType::Int(IntArgs {
                min: Some(10),
                ..IntArgs::default()
            }),
        },
        Field {
            name: "name",
            field_type: FieldType::String(StringArg { max_len: Some(255) }),
        },
    ],
};

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

impl Model for User {
    fn model_struct() -> &'static ModelStruct {
        STRUCT
    }
}
