use sqlx::PgPool;

use crate::modules::items::model::Item;

pub async fn list_items(db: &PgPool) -> Result<Vec<Item>, sqlx::Error> {
    let items = sqlx::query_as::<_, Item>(
        r#"
        SELECT id, name
        FROM items
        ORDER BY id
        "#,
    )
    .fetch_all(db)
    .await?;

    Ok(items)
}
