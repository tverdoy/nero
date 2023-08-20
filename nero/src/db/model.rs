use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub use surrealdb::sql::{Id, Thing};

use crate::db::scheme::Scheme;

use crate::project::DB;

pub type BoxManager = Box<dyn Manager + Send + Sync>;

#[derive(Serialize)]
pub struct Model {
    #[serde(skip)]
    pub manager: BoxManager,
    pub scheme: Scheme,
}

impl Model {
    pub fn new(manager: BoxManager, scheme: Scheme) -> Self {
        Self { manager, scheme }
    }
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[async_trait]
pub trait Manager {
    fn table_name() -> String
    where
        Self: Sized;

    fn thing_from_id(id: Id) -> Thing
    where
        Self: Sized,
    {
        Thing {
            tb: Self::table_name(),
            id,
        }
    }

    fn scheme() -> Scheme
    where
        Self: Sized;

    async fn get(id: Id) -> Self
    where
        Self: Sized;

    async fn create(&mut self);

    async fn delete(self) -> Self
    where
        Self: Sized;

    async fn delete_with_id(id: Id) -> Self
    where
        Self: Sized;

    async fn update(&self);
}

pub struct SurrealDriver<Target>
where
    Target: Serialize + DeserializeOwned + Send + Sync,
{
    marker: PhantomData<Target>,
}

impl<Target> SurrealDriver<Target>
where
    Target: Serialize + DeserializeOwned + Send + Sync,
{
    pub async fn get(id: Thing) -> Target {
        let obj: Option<Target> = DB.select(id).await.unwrap();

        obj.unwrap()
    }

    pub async fn create(thing: Option<Thing>, table_name: String, obj: &Target) -> Id {
        let record: Record = match thing {
            Some(thing) => DB.create(thing).content(obj).await.unwrap(),
            None => DB.create(table_name).content(obj).await.unwrap(),
        };

        record.id.id
    }

    pub async fn delete(thing: Thing) -> Target {
        let target: Option<Target> = DB.delete(thing).await.unwrap();

        target.unwrap()
    }

    //noinspection RsTypeCheck
    pub async fn update(thing: Thing, obj: &Target) {
        let _: Record = DB.update(thing).content(obj).await.unwrap();
    }
}

pub fn format_table_name<T: ToString>(name: T) -> String {
    name.to_string().replace(' ', "").to_lowercase()
}
