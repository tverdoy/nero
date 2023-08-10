use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Model, Serialize, Deserialize, Default, Debug)]
pub struct User {
    #[serde(skip)]
    pub id: Option<Thing>,
    pub name: String,
}
