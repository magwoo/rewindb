use anyhow::Result;
use std::io::{prelude::*, Cursor};
use std::sync::{Arc, RwLock, RwLockReadGuard};

use super::{Memory, MemoryType};

pub struct HeapMemory(Arc<HeapMemoryInner>);

struct HeapMemoryInner {
    data: RwLock<Cursor<Vec<u8>>>,
    wal: RwLock<Cursor<Vec<u8>>>,
    metadata: RwLock<Cursor<Vec<u8>>>,
}

impl Memory for HeapMemory {
    fn read(&self, what: MemoryType) -> Result<RwLockReadGuard<dyn Read>> {
        Ok(match what {
            MemoryType::Metadata => self.0.metadata.read()?,
            MemoryType::Data => self.0.data.read()?,
            MemoryType::Wal => self.0.wal.read()?,
        })
    }
}
