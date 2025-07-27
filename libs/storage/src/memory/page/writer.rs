use std::io::{Seek, SeekFrom, Write};
use std::sync::RwLockWriteGuard;

use super::PAGE_SIZE;

pub struct MemoryPageWriterLock<'a> {
    lock: RwLockWriteGuard<'a, Box<[u8; PAGE_SIZE]>>,
    pos: usize,
}

impl<'a> MemoryPageWriterLock<'a> {
    pub fn new(lock: RwLockWriteGuard<'a, Box<[u8; PAGE_SIZE]>>) -> Self {
        let pos = 0;

        Self { lock, pos }
    }
}

impl<'a> Write for MemoryPageWriterLock<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let slice = self.lock.as_mut_slice();

        let n = buf.len().min(slice.len() - self.pos);
        slice[self.pos..self.pos + n].copy_from_slice(&buf[..n]);
        self.pos += n;

        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> Seek for MemoryPageWriterLock<'a> {
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
