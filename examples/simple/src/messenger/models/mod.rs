use nero::db::model::{Model, Object};

pub mod user;

pub fn build_models() -> Vec<Model> {
    vec![Model::new(
        Box::<user::User>::default(),
        user::User::scheme(),
    )]
}
