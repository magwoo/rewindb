#![allow(unused)]
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::hash::Hasher;
use std::sync::{Arc, Mutex, RwLock};

pub struct KVDatabase(Arc<KVDatabaseInner>);

struct KVDatabaseInner {
    memtable: MemTable,
    sstables: SSTableManager,
    file_pool: FilePool,
    memtable_max_size: u64,
    version: DatabaseVersion,
}

struct MemTable {
    data: RwLock<HashMap<String, String>>,
    max_size: u64,
}

struct SSTableManager {
    sslevels: RwLock<Vec<VecDeque<SSTable>>>,
    max_level: RwLock<u8>,
    level_threshold: RwLock<u8>,
}

struct SSTable {
    file_pool: FilePool,
    blum_filter: BlumFilter,
    cache: Option<MemTable>,
}

struct BlumFilter {}

struct FilePool {}

struct DatabaseVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl DatabaseVersion {
    pub const fn current() -> Self {
        Self {
            major: 0,
            minor: 0,
            patch: 0,
        }
    }

    pub fn is_compatible(&self, other: Self) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}
