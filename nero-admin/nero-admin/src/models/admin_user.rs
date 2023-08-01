use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use nero::db::fieldargs::StringArg;
use nero::db::model::{Field, FieldType, Object, Scheme};
use nero::error::*;
use nero::project::{DB, Settings};
use nero::request::Request;
use nero_util::auth::generate_token;

static STRUCT: &Scheme = &Scheme {
    name: "AdminUser",
    fields: &[
        Field {
            name: "id",
            field_type: FieldType::String(StringArg { max_len: Some(255) }),
        },
        Field {
            name: "username",
            field_type: FieldType::String(StringArg { max_len: Some(255) }),
        },
    ],
};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AdminUser {
    pub id: Option<Thing>,
    pub username: String,
    pub password: String,
}

impl AdminUser {
    pub async fn exists_root() -> bool {
        Self::get("root".into()).await.is_ok()
    }

    pub async fn create_root() -> Result<()> {
        let name = Self::model_struct().name.to_lowercase();
        let admin = AdminUser {
            id: Some(Thing {
                tb: name,
                id: "root".into(),
            }),
            username: "root".to_string(),
            password: "root".to_string(),
        };

        DB.query(
            "create $id set username = $username, password = crypto::bcrypt::generate($password)",
        )
            .bind(admin)
            .await
            .map_err(|e| Error::new(ErrorKind::ObjectCreate, e))?;

        Ok(())
    }

    pub async fn check_login<T: ToString>(&self, password: T) -> Result<bool> {
        let err = |e| Error::new(ErrorKind::Auth, e);

        let res: Option<bool> = DB
            .query("select value crypto::bcrypt::compare(password, $password) as login from $id")
            .bind(("id", &self.id))
            .bind(("password", password.to_string()))
            .await
            .map_err(err)?
            .take(0)
            .map_err(err)?;

        res.ok_or(Error::new_simple(ErrorKind::ObjectGet))
    }

    pub async fn auth(&self, request: &mut Request) -> Result<()> {
        let token = generate_token(
            Settings::admin_auth().exr,
            self.username.clone(),
            &Settings::admin_auth().secret_key,
        )?;
        request
            .set_cookie
            .add("NERO-ADMIN-TOKEN".to_string(), token);

        Ok(())
    }

    pub async fn get_by_username<T: ToString>(username: T) -> Result<Self> {
        let name = Self::model_struct().name.to_lowercase();
        let err = |e| Error::new(ErrorKind::Auth, e);

        let res: Option<Self> = DB
            .query(format!("select * from {name} where username = $username"))
            .bind(("username", username.to_string()))
            .await
            .map_err(err)?
            .take(0)
            .map_err(err)?;

        res.ok_or(Error::new_simple(ErrorKind::ObjectGet))
    }
}

#[async_trait]
impl Object for AdminUser {
    fn model_struct() -> &'static Scheme {
        STRUCT
    }

    async fn init(&self) {
        if !AdminUser::exists_root().await {
            if let Err(e) = AdminUser::create_root().await {
                eprintln!("Failed create admin user: {e}");
            }
        }
    }

    fn get_id(&self) -> Option<Thing> {
        self.id.clone()
    }

    fn set_id(&mut self, id: Thing) {
        self.id = Some(id);
    }
}
