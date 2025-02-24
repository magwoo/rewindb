use anyhow::{bail, Context};
use std::io::Read;

#[derive(Debug)]
pub enum LogAction {
    Insert(Vec<u8>),
    Update(Vec<u8>),
    Delete,
}

#[derive(Debug)]
pub struct Log {
    key: String,
    action: LogAction,
    timestamp: u64,
}

impl Log {
    pub fn from_reader<R: Read>(mut reader: R) -> anyhow::Result<Option<Self>> {
        let mut key_len_buf = [0; size_of::<u32>()];
        reader
            .read(&mut key_len_buf)
            .context("failed to read key len buf")?;

        let key_len = u32::from_le_bytes(key_len_buf);

        if key_len == 0 {
            return Ok(None);
        }

        let mut key_buf = Vec::with_capacity(key_len as usize);
        reader
            .read(&mut key_buf)
            .context("failed to read key buf")?;

        let action = LogAction::from_reader(&mut reader).context("failed to read log action")?;

        let mut timestamp_buf = [0; size_of::<u64>()];
        reader
            .read(&mut timestamp_buf)
            .context("failed to read timestamp buf")?;

        Ok(Some(Self {
            key: String::from_utf8(key_buf).context("failed to parse key as utf-8")?,
            action,
            timestamp: u64::from_le_bytes(timestamp_buf),
        }))
    }
}

impl LogAction {
    pub fn from_reader<R: Read>(mut reader: R) -> anyhow::Result<Self> {
        let mut buf = [0; size_of::<u8>()];

        reader.read(&mut buf).context("failed to read buf")?;

        match u8::from_le_bytes(buf) {
            0 => {
                let mut value_len_buf = [0; size_of::<u64>()];
                reader
                    .read(&mut value_len_buf)
                    .context("failed to read value len buf")?;

                let mut value_buf = Vec::with_capacity(u64::from_le_bytes(value_len_buf) as usize);
                reader
                    .read(&mut value_buf)
                    .context("failed to read value buf")?;

                Ok(Self::Insert(value_buf))
            }
            1 => {
                let mut value_len_buf = [0; size_of::<u64>()];
                reader
                    .read(&mut value_len_buf)
                    .context("failed to read value len buf")?;

                let mut value_buf = Vec::with_capacity(u64::from_le_bytes(value_len_buf) as usize);
                reader
                    .read(&mut value_buf)
                    .context("failed to read value buf")?;

                Ok(Self::Update(value_buf))
            }
            2 => Ok(Self::Delete),
            _ => bail!("unknown log action"),
        }
    }
}
