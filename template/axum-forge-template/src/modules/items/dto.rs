use serde::Serialize;

use crate::modules::items::model::Item;

#[derive(Debug, Serialize)]
pub struct ItemResponse {
    pub id: i64,
    pub name: String,
}

impl From<Item> for ItemResponse {
    fn from(item: Item) -> Self {
        Self {
            id: item.id,
            name: item.name,
        }
    }
}