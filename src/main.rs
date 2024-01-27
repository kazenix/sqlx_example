mod database;

use database::Database;

use database::create_thing::Thing;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let database = Database::new("./db.sqlite").await?;
    database.migrate().await?;

    let thing = Thing { id: 10 };
    let res = database.create_thing(thing).await?;

    Ok(())
}
