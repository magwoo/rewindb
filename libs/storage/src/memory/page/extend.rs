use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Default)]
pub struct MemoryPageExtend {
    inner: Arc<MemoryPageExtendInner>,
}

#[derive(Default)]
struct MemoryPageExtendInner {
    previous_index: AtomicU64,
    next_index: AtomicU64,
}

impl MemoryPageExtend {
    pub fn new(previous_index: Option<u64>, next_index: Option<u64>) -> Self {
        let inner = MemoryPageExtendInner::new(previous_index, next_index);

        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn get_next(&self) -> Option<u64> {
        self.inner.get_next()
    }

    pub fn get_previous(&self) -> Option<u64> {
        self.inner.get_previous()
    }

    pub fn set_next(&self, index: u64) {
        self.inner.set_next(index);
    }

    pub fn set_previous(&self, index: u64) {
        self.inner.set_previous(index);
    }
}

impl MemoryPageExtendInner {
    pub fn new(previous_index: Option<u64>, next_index: Option<u64>) -> Self {
        Self {
            previous_index: AtomicU64::new(previous_index.unwrap_or_default()),
            next_index: AtomicU64::new(next_index.unwrap_or_default()),
        }
    }

    pub fn get_next(&self) -> Option<u64> {
        match self.next_index.load(Ordering::Acquire) {
            0 => None,
            index => Some(index),
        }
    }

    pub fn get_previous(&self) -> Option<u64> {
        match self.previous_index.load(Ordering::Acquire) {
            0 => None,
            index => Some(index),
        }
    }

    pub fn set_next(&self, index: u64) {
        self.next_index.store(index, Ordering::Release);
    }

    pub fn set_previous(&self, index: u64) {
        self.previous_index.store(index, Ordering::Release);
    }
}
