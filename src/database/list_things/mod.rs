use super::*;

impl Database {
    pub async fn list_things(&self) -> Result<Vec<Thing>, Error> {
        // I use a separate file for my sql here using include_str so I get better
        // syntax highlighting behavior for the sql.
        let result = sqlx::query_as::<_, Thing>(include_str!("list_things.sql"))
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }
}
