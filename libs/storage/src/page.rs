use std::io::{Read, Seek, Write};

pub mod stream;

pub trait StoragePage: Clone + Sync {
    const SIZE: u64 = 4096;

    type ReaderLock<'a>: Read + Seek
    where
        Self: 'a;
    type WriterLock<'a>: Write + Seek
    where
        Self: 'a;

    fn reader<'a>(&'a self) -> Self::ReaderLock<'a>;

    fn writer<'a>(&'a self) -> Self::WriterLock<'a>;
}
