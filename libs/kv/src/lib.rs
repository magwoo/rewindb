use anyhow::Context;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::io::{Cursor, Seek, SeekFrom};
use std::path::PathBuf;

pub struct Database {
    file: File,
}

impl Database {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .context("failed to open file")?;

        Ok(Self { file })
    }

    pub fn set(&mut self, key: &str, value: &str) -> anyhow::Result<()> {
        self.file
            .seek(SeekFrom::End(0))
            .context("failed to seek at end")?;

        let mut buf = Cursor::new(Vec::new());

        buf.write_all(&(key.len() as u32).to_le_bytes())?;
        buf.write_all(&(value.len() as u64).to_le_bytes())?;
        buf.write_all(key.as_bytes())?;
        buf.write_all(value.as_bytes())?;

        self.file
            .write(buf.into_inner().as_ref())
            .context("failed to write to file")?;

        self.file.flush().context("failed to flush file")?;

        Ok(())
    }

    pub fn get(&mut self, key: &str) -> anyhow::Result<Option<String>> {
        self.file
            .seek(SeekFrom::Start(0))
            .context("failed to seek to start")?;

        let mut reader = BufReader::new(&self.file);

        loop {
            let mut key_len_bytes = [0; size_of::<u32>()];
            if let Err(e) = reader.read_exact(&mut key_len_bytes) {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    return Ok(None);
                }
                return Err(e.into());
            }

            let mut value_len_bytes = [0; size_of::<u64>()];
            reader
                .read_exact(&mut value_len_bytes)
                .context("failed to read value length")?;

            let key_len = u32::from_le_bytes(key_len_bytes) as usize;
            let value_len = u64::from_le_bytes(value_len_bytes) as usize;

            let mut key_bytes = vec![0; key_len];
            reader
                .read_exact(&mut key_bytes)
                .context("failed to read key bytes")?;

            let record_key =
                String::from_utf8(key_bytes).context("failed to parse key as UTF-8")?;

            let mut value_bytes = vec![0; value_len];
            reader
                .read_exact(&mut value_bytes)
                .context("failed to read value bytes")?;
            if record_key == key {
                let record_value =
                    String::from_utf8(value_bytes).context("failed to parse value as UTF-8")?;

                return Ok(Some(record_value));
            }
        }
    }
}
