use anyhow::{bail, Context, Result};
use std::fs::File;
use std::io::{prelude::*, Cursor};
use std::path::Path;

use self::version::InstanceVersion;

mod version;

#[derive(Debug)]
pub struct InstanceMetadata {
    f: File,
    version: InstanceVersion,
    database_names: Vec<String>,
}

impl InstanceMetadata {
    pub fn databases(&self) -> &[String] {
        &self.database_names
    }

    pub fn new<R: Read>(reader: &mut R) -> Result<Self> {
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .with_context(|| format!("Error while open metadata file {path:?}"))?;

        let version =
            InstanceVersion::from_reader(&mut file).context("Reading version from file error")?;

        let mut len_buf = [0; 4];
        let readed = file
            .read(&mut len_buf)
            .context("Read database names len buf error")?;

        let database_names = if readed != len_buf.len() {
            Vec::new()
        } else {
            let len = u32::from_le_bytes(len_buf);

            let mut buf = vec![0; len as usize];
            file.read_exact(&mut buf)
                .context("Read database names buf error");

            let mut buf = Cursor::new(buf);
            let mut names = Vec::new();

            loop {
                let mut len_buf = [0; 2];
                let readed = buf
                    .read(&mut len_buf)
                    .expect("Database names read name len buf error");

                if readed != len_buf.len() {
                    break;
                }

                let len = u16::from_le_bytes(len_buf);

                let mut name_buf = vec![0; len as usize];
                buf.read_exact(&mut name_buf)
                    .context("Read database name error")?;

                names.push(String::from_utf8_lossy(&name_buf).to_string());
            }

            names
        };

        if version != InstanceVersion::current() {
            bail!("Data has difference version!");
        }

        Ok(Self {
            f: file,
            version,
            database_names,
        })
    }

    pub fn save(&mut self) -> Result<()> {
        self.f
            .seek(std::io::SeekFrom::Start(0))
            .context("failed to seek")?;

        self.version
            .save_to_writer(&mut self.f)
            .context("failed to save version to metadata")?;

        if self.database_names.is_empty() {
            self.f.write_all(&0_u32.to_le_bytes())
        } else {
            let mut total_len = (self.database_names.len() * 2) as u32;

            self.database_names
                .iter()
                .for_each(|n| total_len += n.bytes().len() as u32);

            self.f.write_all(&total_len.to_le_bytes())
        }
        .context("failed to write database names len")?;

        for name in self.database_names.iter() {
            self.f
                .write_all(name.as_bytes())
                .with_context(|| format!("failed to save database name({name})"))?;
        }

        self.f.flush().context("failed to flush metadata");

        Ok(())
    }
}
