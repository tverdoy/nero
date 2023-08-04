use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use nero::db::model::{format_db_name, Object};
use nero::db::scheme::{Field, FieldArg, FieldType, Scheme};
use nero::error::*;
use nero::project::DB;
use nero::request::Request;
use nero::settings::Settings;
use nero_util::auth::{generate_token, verify_token};
use nero_util::http::AuthType;

const MODEL_NAME: &str = "Admin user";

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AdminUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl AdminUser {
    pub async fn check_auth(request: &Request) -> Result<Self> {
        if let Some(AuthType::Bearer(token)) = &request.head.auth {
            let sub = verify_token(token, &Settings::admin_auth().secret_key)
                .map_err(|e| Error::new(ErrorKind::Auth, e))?;

            Self::get(sub.into()).await
        } else {
            Err(Error::new_simple(ErrorKind::TokenIsNone))
        }
    }

    pub async fn exists_root() -> bool {
        Self::get("root".into()).await.is_ok()
    }

    pub async fn create_root() -> Result<()> {
        let name = Self::name().to_lowercase();
        let admin = AdminUser {
            id: Some(Thing {
                tb: name,
                id: "root".into(),
            }),
            username: "root".to_string(),
            password: Some("root".to_string()),
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

    pub async fn auth(&self) -> Result<String> {
        generate_token(
            Settings::admin_auth().exr,
            self.username.clone(),
            &Settings::admin_auth().secret_key,
        )
            .map_err(|e| e.into())
    }

    pub async fn get_by_username<T: ToString>(username: T) -> Result<Self> {
        let name = format_db_name(Self::name());
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
                Field::new(
                    "username",
                    FieldType::String,
                    vec![FieldArg::MaxLength(255)],
                ),
                Field::new(
                    "password",
                    FieldType::String,
                    vec![FieldArg::MaxLength(255)],
                ),
            ],
        )
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
