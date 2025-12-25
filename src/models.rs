use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateItem {
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: usize,
    pub name: String,
}