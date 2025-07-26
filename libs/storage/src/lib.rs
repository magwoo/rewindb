use anyhow::Context;

use self::page::StoragePage;
use self::page::stream::PageStream;

pub mod page;

pub trait Storage: Clone {
    const PAGE_SIZE: u64;

    type Page: StoragePage;

    fn new_page(&self) -> anyhow::Result<u64>;

    fn extend_page(&self, extend_index: u64, contiguous: bool) -> anyhow::Result<u64>;

    fn get_page(&self, index: u64) -> anyhow::Result<&Self::Page>;

    fn flush(&self) -> anyhow::Result<()>;

    fn new_batch(&self, mut len: u64, contiguous: bool) -> anyhow::Result<u64> {
        len = len.saturating_sub(Self::PAGE_SIZE);
        let first_index = self.new_page().context("failed to create first page")?;

        let mut last_index = first_index;

        while len > 0 {
            len = len.saturating_sub(Self::PAGE_SIZE);

            last_index = self
                .extend_page(last_index, contiguous)
                .context("failed to create page extends index: {last_index}")?;
        }

        Ok(first_index)
    }

    fn page_stream(&self, start_index: u64) -> PageStream<Self> {
        PageStream::new(self.clone(), start_index)
    }
}
