use nero::db::model::Model;

pub mod user;

pub fn build_models() -> Vec<Model> {
    vec![Box::<user::User>::default()]
}
