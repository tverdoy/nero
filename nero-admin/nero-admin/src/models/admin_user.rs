use serde::{Deserialize, Serialize};
use surrealdb::sql::Id;
use nero::db::fieldargs::{IntArgs, StringArg};
use nero::db::model::{Field, FieldType, Object, Scheme};

static STRUCT: &Scheme = &Scheme {
    name: "AdminUser",
    fields: &[
        Field {
            name: "id",
            field_type: FieldType::String(StringArg { max_len: Some(255) })
        },
        Field {
            name: "username",
            field_type: FieldType::String(StringArg { max_len: Some(255) }),
        },
    ],
};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AdminUser {
    #[serde(skip)]
    pub id: Option<Id>,
    pub username: String,
    pub password: String
}

impl Object for AdminUser {
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
