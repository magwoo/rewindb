use std::fs::File;

use self::sstable::SSTable;

mod sstable;

#[derive(Debug)]
pub struct Database {
    f: File,
    wal: File,
    memtable: SSTable,
    transactions: Vec<File>,
    cache: Vec<SSTable>,
}

impl Database {
    pub fn new(f: File, wal: File) -> Self {
        Self {
            f,
            wal,
            memtable: SSTable::default(),
            transactions: Vec::new(),
            cache: Vec::new(),
        }
    }
}
