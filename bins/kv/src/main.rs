use anyhow::Context;
use rewind_kv::Database;

fn main() -> anyhow::Result<()> {
    let mut database = Database::new("./test.data".into()).context("failed to open database")?;

    database
        .set("key1", "value1")
        .context("failed to set key")?;

    let value = database.get("key1").context("failed to get value")?;

    println!("value: {value:?}");

    Ok(())
}
