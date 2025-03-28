use std::collections::HashMap;
use uuid::Uuid;


pub fn get_default_repositories() -> String {
    "OCA/OCB\nOCA/l10n-spain\nodoo/odoo\nleptos-rs/leptos".to_string()
}
pub fn get_default_url_repository() -> String {
    "<repository>".to_string()
}
pub fn get_default_url_model() -> String {
    "<model>".to_string()
}
pub fn get_default_url_id() -> String {
    "<id>".to_string()
}

#[derive(Clone, Default, Debug)]
pub struct GlobalState {
    pub message: String,
    pub buttons: HashMap<String, bool>,
    pub model: String,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RepositoryButton {
    pub id: Uuid,
    pub label: String,
    pub active: bool,
}
impl RepositoryButton {
    pub fn new(id: Uuid, label: String, active: bool) -> Self {
        Self { id, label, active }
    }
}