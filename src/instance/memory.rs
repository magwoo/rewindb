use anyhow::{Context, Result};
use std::collections::HashMap;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

mod file;
mod heap;

const MAX_READERS: usize = 8;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum MemoryType {
    Metadata,
}

trait Memory: Read + Write + Sized {
    type PathBase;

    fn get_for_read(what: MemoryType) -> Result<Self>;

    fn get_for_write(what: MemoryType) -> Result<Self>;
}

pub struct RwPool<M: Memory> {
    path_base: M::PathBase,
    readers: Mutex<HashMap<MemoryType, Vec<Arc<Mutex<M>>>>>,
    writers: Mutex<HashMap<MemoryType, Arc<Mutex<M>>>>,
}

impl<M: Memory> RwPool<M> {
    pub fn new(path: M::PathBase) -> Self {
        Self {
            path_base: path,
            readers: Mutex::new(HashMap::new()),
            writers: Mutex::new(HashMap::new()),
        }
    }

    pub fn read(&self, what: MemoryType) -> Result<Arc<Mutex<M>>> {
        let mut map = self.readers.lock().unwrap();

        if let Some(readers) = map.get_mut(&what) {
            let reader = readers.iter().find(|r| r.is_poisoned());

            match reader {
                Some(reader) => Ok(reader.clone()),
                None => {
                    if readers.len() < MAX_READERS {
                        let reader = Arc::new(Mutex::new(
                            M::get_for_read(what).context("failed to open new reader")?,
                        ));
                        readers.push(reader.clone());
                        Ok(reader)
                    } else {
                        Ok(readers
                            .last()
                            .expect("expected items after len check")
                            .clone())
                    }
                }
            }
        } else {
            let reader = Arc::new(Mutex::new(
                M::get_for_read(what).context("failed to create new reader")?,
            ));

            map.insert(what, vec![reader.clone()]);

            Ok(reader)
        }
    }

    pub fn write(&self, what: MemoryType) -> Result<Arc<Mutex<M>>> {
        let mut map = self.writers.lock().unwrap();

        match map.get(&what) {
            Some(writer) => Ok(writer.clone()),
            None => {
                let writer = Arc::new(Mutex::new(
                    M::get_for_write(what).context("failed to open for write")?,
                ));

                map.insert(what, writer.clone());

                Ok(writer)
            }
        }
    }
}
