use super::*;

#[derive(Debug, sqlx::FromRow)]
pub struct Thing {
    pub id: i64,
}

impl Database {
    pub async fn create_thing(&self, arg1: Thing) -> Result<Thing, Error> {
        // I use a separate file for my sql here using include_str so I get better
        // syntax highlighting behavior for the sql.
        let result = sqlx::query_as::<_, Thing>(include_str!("create_thing.sql"))
            .bind(arg1.id)
            .fetch_one(&self.pool)
            .await?;

        Ok(result)
    }
}
