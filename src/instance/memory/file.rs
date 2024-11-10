use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, Cursor};
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use super::{Memory, MemoryType};

#[derive(Debug)]
pub struct FsMemoryPool {
    path_base: PathBuf,
    max_readers: NonZeroUsize,
    readers: Mutex<HashMap<MemoryType, Vec<Arc<Mutex<File>>>>>,
    writers: Mutex<HashMap<MemoryType, Arc<Mutex<File>>>>,
}

impl Memory for FsMemoryPool {
    fn get_for_read(&self, _what: MemoryType) -> Result<Arc<Mutex<dyn Read>>> {
        Ok(Arc::new(Mutex::new(Cursor::new(vec![]))))
    }

    fn get_for_write(&self, _what: MemoryType) -> Result<Arc<Mutex<dyn Write>>> {
        Ok(Arc::new(Mutex::new(Cursor::new(vec![]))))
    }
}

impl FsMemoryPool {
    pub fn new(path: PathBuf, max_readers: NonZeroUsize) -> Self {
        Self {
            path_base: path,
            max_readers,
            readers: Mutex::new(HashMap::new()),
            writers: Mutex::new(HashMap::new()),
        }
    }
}
