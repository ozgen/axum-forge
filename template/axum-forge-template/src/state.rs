use sqlx::PgPool;
use std::sync::Arc;

use crate::modules::items::repo::{ItemRepository, PgItemRepository};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub items_repo: Arc<dyn ItemRepository>,
}

#[allow(dead_code)]
impl AppState {
    pub fn new(db: PgPool) -> Self {
        let items_repo: Arc<dyn ItemRepository> = Arc::new(PgItemRepository::new(db.clone()));

        Self { db, items_repo }
    }

    pub fn with_repo(db: PgPool, items_repo: Arc<dyn ItemRepository>) -> Self {
        Self { db, items_repo }
    }
}
