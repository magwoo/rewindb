use std::io::prelude::*;
use std::io::Cursor;

use super::Memory;

pub struct InMemory(Cursor<Vec<u8>>);

impl Memory for InMemory {
    type PathBase = ();

    fn get_for_read(_: super::MemoryType) -> anyhow::Result<Self> {
        Ok(Self(Cursor::new(Vec::new())))
    }

    fn get_for_write(_: super::MemoryType) -> anyhow::Result<Self> {
        Ok(Self(Cursor::new(Vec::new())))
    }
}

impl Read for InMemory {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl Write for InMemory {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
