use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Cursor;
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::bail;
use anyhow::Result;

use super::{Memory, MemoryType};

pub struct HeapMemory(Mutex<HashMap<MemoryType, Arc<Mutex<Cursor<Vec<u8>>>>>>);

impl HeapMemory {
    pub fn new() -> Self {
        Self(Mutex::new(HashMap::new()))
    }
}

impl Memory for HeapMemory {
    fn get_for_read(&self, what: MemoryType) -> Result<Arc<Mutex<dyn Read>>> {
        let mut map = match self.0.lock() {
            Ok(map) => map,
            _ => bail!("failed to open mutex"),
        };

        if let Some(reader) = map.get(&what) {
            Ok(reader.clone())
        } else {
            let reader = Arc::new(Mutex::new(Cursor::new(Vec::new())));

            map.insert(what, reader.clone());

            Ok(reader)
        }
    }

    fn get_for_write(&self, what: MemoryType) -> Result<Arc<Mutex<dyn Write>>> {
        let mut map = match self.0.lock() {
            Ok(map) => map,
            _ => bail!("failed to open mutex"),
        };

        if let Some(reader) = map.get(&what) {
            Ok(reader.clone())
        } else {
            let reader = Arc::new(Mutex::new(Cursor::new(Vec::new())));

            map.insert(what, reader.clone());

            Ok(reader)
        }
    }
}
