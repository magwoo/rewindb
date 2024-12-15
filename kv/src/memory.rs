use anyhow::Result;
use std::io::prelude::*;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub mod heap;

pub enum MemoryType {
    Metadata,
    Data,
    Wal,
}

pub trait Memory {
    fn read(&self) -> Result<RwLockReadGuard<dyn Read>>;

    fn write(&self) -> Result<RwLockWriteGuard<dyn Write>>;
}
