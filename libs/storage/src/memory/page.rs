use extend::MemoryPageExtend;
use std::sync::{Arc, RwLock};

use self::reader::MemoryPageReaderLock;
use self::writer::MemoryPageWriterLock;
use crate::page::StoragePage;

pub mod extend;
pub mod reader;
pub mod writer;

const PAGE_SIZE: usize = 4096;

#[derive(Clone)]
pub struct MemoryPage {
    extends: MemoryPageExtend,
    data: Arc<RwLock<Box<[u8; PAGE_SIZE]>>>,
}

impl MemoryPage {
    pub fn new(previous: Option<u64>) -> anyhow::Result<Self> {
        let extends = MemoryPageExtend::new(previous, None);
        let data = Arc::new(RwLock::new(Box::new([0; PAGE_SIZE])));

        Ok(Self { extends, data })
    }

    pub fn set_next_index(&self, index: u64) {
        self.extends.set_next(index);
    }
}

impl StoragePage for MemoryPage {
    const SIZE: u64 = PAGE_SIZE as u64;

    type ReaderLock<'a> = MemoryPageReaderLock<'a>;
    type WriterLock<'a> = MemoryPageWriterLock<'a>;

    fn reader<'a>(&'a self) -> Self::ReaderLock<'a> {
        let lock = self.data.read().unwrap();

        MemoryPageReaderLock::new(lock)
    }

    fn writer<'a>(&'a self) -> Self::WriterLock<'a> {
        let lock = self.data.write().unwrap();

        MemoryPageWriterLock::new(lock)
    }
}
