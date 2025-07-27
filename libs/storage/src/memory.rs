use std::sync::{Arc, RwLock};

use anyhow::Context;

use self::page::MemoryPage;
use crate::Storage;

pub mod page;

#[derive(Clone)]
pub struct MemoryStorage {
    pages: Arc<RwLock<Vec<MemoryPage>>>,
}

impl Storage for MemoryStorage {
    type Page = MemoryPage;

    fn new_page(&self) -> anyhow::Result<u64> {
        let mut pages = self.pages.write().unwrap();

        let new_page_index = pages.len() as u64;
        let page = MemoryPage::new(None).context("failed to create page")?;

        pages.push(page);

        Ok(new_page_index)
    }

    fn new_page_extended(&self, previous_index: u64, contiguous: bool) -> anyhow::Result<u64> {
        let mut pages = self.pages.write().unwrap();

        let new_page_index = pages.len() as u64;

        if contiguous && previous_index + 1 != new_page_index {
            anyhow::bail!(
                "new page cannot be contiguous with previous page, previous index: {previous_index}, new index: {new_page_index}"
            );
        }

        pages
            .get_mut(previous_index as usize)
            .context("failed to get previous page")?
            .set_next_index(new_page_index);

        let new_page =
            MemoryPage::new(Some(previous_index)).context("failed to create new page")?;

        pages.push(new_page);

        Ok(new_page_index)
    }

    fn get_page(&self, index: u64) -> anyhow::Result<Option<Self::Page>> {
        let pages = self.pages.read().unwrap();

        Ok(pages.get(index as usize).cloned())
    }

    fn flush(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
