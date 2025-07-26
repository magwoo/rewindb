use std::io::{Read, Write};

pub mod stream;

pub trait StoragePage: Read + Write {
    fn index(&self) -> u64;
}
