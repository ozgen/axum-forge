use async_trait::async_trait;
use sqlx::PgPool;

use crate::modules::items::model::Item;

#[async_trait]
pub trait ItemRepository: Send + Sync {
    async fn list_items(&self) -> Result<Vec<Item>, sqlx::Error>;
}

pub struct PgItemRepository {
    pool: PgPool,
}

impl PgItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ItemRepository for PgItemRepository {
    async fn list_items(&self) -> Result<Vec<Item>, sqlx::Error> {
        let items = sqlx::query_as::<_, Item>(
            r#"
            SELECT id, name
            FROM items
            ORDER BY id
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub struct FakeItemRepository {
    pub items: Vec<Item>,
}

#[cfg(test)]
#[async_trait::async_trait]
impl ItemRepository for FakeItemRepository {
    async fn list_items(&self) -> Result<Vec<Item>, sqlx::Error> {
        Ok(self.items.clone())
    }
}