use super::*;

impl Database {
    pub async fn delete_thing(&self, id: i64) -> Result<Thing, Error> {
        // I use a separate file for my sql here using include_str so I get better
        // syntax highlighting behavior for the sql.
        let result = sqlx::query_as::<_, Thing>(include_str!("delete_thing.sql"))
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(result)
    }
}
