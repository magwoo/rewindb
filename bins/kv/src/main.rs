use anyhow::Context;
use rewind_kv::Database;

fn main() -> anyhow::Result<()> {
    let mut database = Database::new("./test.data".into()).context("failed to open database")?;

    database.set("test", "123").context("failed to set key")?;

    Ok(())
}
