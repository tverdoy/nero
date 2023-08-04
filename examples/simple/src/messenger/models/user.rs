use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use nero::db::model::Object;
use nero::db::scheme::{Field, FieldArg, FieldType, Scheme};

const MODEL_NAME: &str = "User";

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    #[serde(skip)]
    pub id: Option<Thing>,
    pub name: String,
}

impl Object for User {
    fn name() -> &'static str
        where
            Self: Sized,
    {
        MODEL_NAME
    }

    fn scheme() -> Scheme {
        Scheme::new(
            MODEL_NAME,
            vec![
                Field::new("id", FieldType::String, vec![]),
                Field::new("name", FieldType::String, vec![FieldArg::MaxLength(255)]),
            ],
        )
    }

    fn get_id(&self) -> Option<Thing> {
        self.id.clone()
    }

    fn set_id(&mut self, id: Thing) {
        self.id = Some(id);
    }
}
