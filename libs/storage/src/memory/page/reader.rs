use std::io::{Read, Seek, SeekFrom};
use std::sync::RwLockReadGuard;

use super::PAGE_SIZE;

pub struct MemoryPageReaderLock<'a> {
    lock: RwLockReadGuard<'a, Box<[u8; PAGE_SIZE]>>,
    pos: usize,
}

impl<'a> MemoryPageReaderLock<'a> {
    pub fn new(lock: RwLockReadGuard<'a, Box<[u8; PAGE_SIZE]>>) -> Self {
        let pos = 0;

        Self { lock, pos }
    }
}

impl<'a> Read for MemoryPageReaderLock<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let slice = self.lock.as_slice();

        let n = buf.len().min(slice.len() - self.pos);
        buf[..n].copy_from_slice(&slice[self.pos..self.pos + n]);
        self.pos += n;

        Ok(n)
    }
}

impl<'a> Seek for MemoryPageReaderLock<'a> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(pos) => pos as i64,
            SeekFrom::End(offset) => PAGE_SIZE as i64 + offset,
            SeekFrom::Current(offset) => self.pos as i64 + offset,
        };

        if new_pos < 0 || new_pos > PAGE_SIZE as i64 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "seek position out of bounds, pos: {pos:?}, current pos: {}",
                    self.pos
                ),
            ));
        }

        self.pos = new_pos as usize;
        Ok(self.pos as u64)
    }
}
