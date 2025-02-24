use std::fs::File;
use std::io::prelude::*;
use std::io::{Cursor, Seek, SeekFrom};
use std::path::PathBuf;

use anyhow::Context;

pub struct Database {
    file: File,
}

impl Database {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let file = File::options()
            .read(true)
            .write(true)
            .open(path)
            .context("failed to open file")?;

        Ok(Self { file })
    }

    pub fn set(&mut self, key: &str, value: &str) -> anyhow::Result<()> {
        self.file
            .seek(SeekFrom::End(0))
            .context("failed to seek at end")?;

        let mut buf = Cursor::new(Vec::new());

        buf.write_all(&(key.len() as u64).to_le_bytes())?;
        buf.write_all(&(value.len() as u64).to_le_bytes())?;
        buf.write_all(key.as_bytes())?;
        buf.write_all(value.as_bytes())?;

        self.file
            .write(buf.into_inner().as_ref())
            .context("failed to write to file")?;

        Ok(())
    }
}
