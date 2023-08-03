use crate::messenger::models::user::USER_SCHEME;
use nero::db::model::Model;

pub mod user;

pub fn build_models() -> Vec<Model> {
    vec![Model::new(Box::<user::User>::default(), USER_SCHEME)]
}
