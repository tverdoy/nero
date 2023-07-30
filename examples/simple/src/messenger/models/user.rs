use nero::db::fieldargs::{IntArgs, StringArg};
use nero::db::model::{Field, FieldType, Object, Scheme};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Id;

static STRUCT: &Scheme = &Scheme {
    name: "User",
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

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    #[serde(skip)]
    pub id: Option<Id>,
    pub name: String,
}

impl Object for User {
    fn model_struct() -> &'static Scheme {
        STRUCT
    }

    fn get_id(&self) -> Option<Id> {
        self.id.clone()
    }

    fn set_id(&mut self, id: Id) {
        self.id = Some(id);
    }
}
