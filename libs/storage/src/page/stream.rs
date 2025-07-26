use crate::Storage;

pub struct PageStream<S: Storage> {
    start_index: u64,
    current_index: u64,
    storage: S,
}

impl<S: Storage> PageStream<S> {
    pub fn new(storage: S, index: u64) -> Self {
        Self {
            start_index: index,
            current_index: index,
            storage,
        }
    }
}
