use anyhow::Result;
use std::fmt::Debug;
use std::io::prelude::*;
use std::sync::{Arc, LazyLock, Mutex};

use self::file::FsPool;

pub mod file;
pub mod heap;

pub static MEMORY: LazyLock<Box<dyn Memory>> = LazyLock::new(|| {
    Box::new(FsPool::new(
        "./data".try_into().unwrap(),
        8.try_into().unwrap(),
    ))
});

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MemoryType {
    Metadata,
}

pub trait Memory: Sync + Send {
    fn get_for_read(&self, what: MemoryType) -> Result<Arc<Mutex<dyn Read>>>;

    fn get_for_write(&self, what: MemoryType) -> Result<Arc<Mutex<dyn Write>>>;
}

// pub fn read(&self, what: MemoryType) -> Result<Arc<Mutex<File>>> {
//     let mut map = self.readers.lock().unwrap();

//     if let Some(readers) = map.get_mut(&what) {
//         let reader = readers.iter().find(|r| r.is_poisoned());

//         match reader {
//             Some(reader) => Ok(reader.clone()),
//             None => {
//                 if readers.len() < MAX_READERS {
//                     let reader = Arc::new(Mutex::new(
//                         M::get_for_read(what).context("failed to open new reader")?,
//                     ));
//                     readers.push(reader.clone());
//                     Ok(reader)
//                 } else {
//                     Ok(readers
//                         .last()
//                         .expect("expected items after len check")
//                         .clone())
//                 }
//             }
//         }
//     } else {
//         let reader = Arc::new(Mutex::new(
//             M::get_for_read(what).context("failed to create new reader")?,
//         ));

//         map.insert(what, vec![reader.clone()]);

//         Ok(reader)
//     }
// }

// pub fn write(&self, what: MemoryType) -> Result<Arc<Mutex<M>>> {
//     let mut map = self.writers.lock().unwrap();

//     match map.get(&what) {
//         Some(writer) => Ok(writer.clone()),
//         None => {
//             let writer = Arc::new(Mutex::new(
//                 M::get_for_write(what).context("failed to open for write")?,
//             ));

//             map.insert(what, writer.clone());

//             Ok(writer)
//         }
//     }
// }
