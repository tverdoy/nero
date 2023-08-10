use crate::nero::db::model::Manager;
use nero::db::model::Model;

pub mod user;

pub fn build_models() -> Vec<Model> {
    vec![Model::new(
        Box::<user::User>::default(),
        user::User::scheme(),
    )]
}
