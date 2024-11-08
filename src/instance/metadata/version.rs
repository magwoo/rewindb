use anyhow::{Context, Result};
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct InstanceVersion {
    major: u16,
    minor: u16,
    patch: u16,
    pre: Option<String>,
}

impl InstanceVersion {
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut len_buf = [0; 1];
        let len = reader
            .read(&mut len_buf)
            .context("Error while read metadata version len buffer")?;

        if len == 0 {
            return Ok(Self::current());
        }

        let len = u8::from_le_bytes(len_buf);

        let mut buf = vec![0; (len as usize) + 6];
        reader
            .read_exact(&mut buf)
            .context("Error while read metadata buffer")?;

        let major = u16::from_le_bytes(buf[..2].try_into().expect("Missing major bytes"));
        let minor = u16::from_le_bytes(buf[2..4].try_into().expect("Missing minor bytes"));
        let patch = u16::from_le_bytes(buf[4..6].try_into().expect("Missing patch bytes"));

        let pre = if len == 0 {
            None
        } else {
            Some(String::from_utf8_lossy(&buf[6..]).to_string())
        };

        Ok(Self {
            major,
            minor,
            patch,
            pre,
        })
    }

    pub fn current() -> Self {
        Self {
            major: 0,
            minor: 0,
            patch: 0,
            pre: Some("alpha".to_string()),
        }
    }

    pub fn save_to_writer<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self.pre.as_ref() {
            Some(pre) => writer.write_all(&(pre.as_bytes().len() as u8).to_le_bytes()),
            None => writer.write_all(&0_u8.to_le_bytes()),
        }
        .context("failed to write pre version len")?;

        writer
            .write_all(&self.major.to_le_bytes())
            .context("failed to write major")?;

        writer
            .write_all(&self.minor.to_le_bytes())
            .context("failed to write minor")?;

        writer
            .write_all(&self.patch.to_le_bytes())
            .context("failed to write patch")?;

        if let Some(pre) = self.pre.as_ref() {
            writer
                .write_all(pre.as_bytes())
                .context("failed to write pre version")?;
        }

        Ok(())
    }
}
