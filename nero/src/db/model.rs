use crate::db::fieldargs::{IntArgs, StringArg};
use crate::error::*;
use crate::project::DB;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

pub type BoxObject = Box<dyn Object + Send + Sync>;

pub struct Model {
    pub object: BoxObject,
    pub scheme: &'static Scheme,
}

impl Model {
    pub fn new(object: BoxObject, scheme: &'static Scheme) -> Self {
        Self { object, scheme }
    }
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[async_trait]
pub trait Object {
    fn name() -> &'static str
    where
        Self: Sized;

    async fn init(&self) {}

    fn get_id(&self) -> Option<Thing>;

    fn set_id(&mut self, id: Thing);

    async fn create(&mut self) -> Result<()>
    where
        Self: Serialize + Sized + Sync,
    {
        let name = Self::name().to_lowercase();
        let err = |e| Error::new(ErrorKind::ObjectCreate, e);

        let record: Record = match self.get_id() {
            Some(id) => DB.create((name, id)).content(&self).await,
            None => DB.create(name).await,
        }
        .map_err(err)?;

        self.set_id(record.id);

        Ok(())
    }

    async fn get(id: Id) -> Result<Self>
    where
        Self: DeserializeOwned + Sync,
    {
        let name = Self::name().to_lowercase();

        let obj: Option<Self> = DB
            .select((name, id))
            .await
            .map_err(|e| Error::new(ErrorKind::ObjectGet, e))?;

        obj.ok_or(Error::new_simple(ErrorKind::ObjectNotExists))
    }

    async fn delete(id: Id) -> Result<Self>
    where
        Self: DeserializeOwned + Sync,
    {
        let name = Self::name().to_lowercase();

        let obj: Option<Self> = DB
            .delete((name, id))
            .await
            .map_err(|e| Error::new(ErrorKind::ObjectDelete, e))?;

        obj.ok_or(Error::new_simple(ErrorKind::ObjectNotExists))
    }

    async fn update(&self) -> Result<Thing>
    where
        Self: Serialize + Sync + Sized,
    {
        let name = Self::name().to_lowercase();

        let id = self
            .get_id()
            .ok_or(Error::new_simple(ErrorKind::ObjectIdIsNone))?;

        let record: Record = DB
            .update((name, id))
            .content(&self)
            .await
            .map_err(|e| Error::new(ErrorKind::ObjectUpdate, e))?;

        Ok(record.id)
    }

    async fn merge<M>(&self, merge: M) -> Result<Thing>
    where
        Self: Serialize + Sized + Sync,
        M: Serialize + Send,
    {
        let name = Self::name().to_lowercase();
        let id = self
            .get_id()
            .ok_or(Error::new_simple(ErrorKind::ObjectIdIsNone))?;

        let record: Record = DB
            .update((name, id))
            .merge(merge)
            .await
            .map_err(|e| Error::new(ErrorKind::ObjectMerge, e))?;

        Ok(record.id)
    }
}

#[derive(Serialize)]
pub struct Scheme {
    pub name: &'static str,
    pub fields: &'static [Field],
}

#[derive(Serialize)]
pub struct Field {
    pub name: &'static str,
    pub field_type: FieldType,
}

#[derive(Serialize)]
pub enum FieldType {
    Int(IntArgs),
    String(StringArg),
    Bool,
    LinkTo,
}
