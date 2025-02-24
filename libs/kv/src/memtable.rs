use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use anyhow::Context;

use crate::row::Log;

#[derive(Debug)]
pub struct MemTable {
    file: Arc<File>,
    logs: VecDeque<Log>,
}

impl MemTable {
    pub fn from_reader<R: Read>(mut reader: R) -> anyhow::Result<Self> {
        let mut logs = VecDeque::new();

        while let Some(log) = Log::from_reader(&mut reader).context("failed to read log")? {
            logs.push_back(log);
        }

        Ok(Self { logs })
    }
}
