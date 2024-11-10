use anyhow::{Context, Result};
use std::fs::{DirBuilder, File};
use std::path::PathBuf;

use self::metadata::InstanceMetadata;
use crate::database::Database;
use crate::memory::prelude::*;

pub mod memory;
mod metadata;

#[derive(Debug)]
struct Instance {
    databases: Vec<Database>,
    metadata: InstanceMetadata,
}

impl Instance {
    pub fn new(path_base: &str) -> Result<Self> {
        let metadata =
            InstanceMetadata::new(&metadata_path).context("Error while load instance metadata")?;

        let mut instance = Self {
            path: dir_path,
            databases: Vec::new(),
            metadata,
        };

        instance
            .load_databases()
            .context("failed to load instance databases");

        Ok(instance)
    }

    fn load_databases(&mut self) -> Result<()> {
        for name in self.metadata.databases().iter() {
            let mut data_path = self.path.clone();
            data_path.push(format!("{}.data", name));

            let data_file = File::options()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(&data_path)
                .with_context(|| format!("failed to open/create data file({data_path:?})"))?;

            let mut wal_path = self.path.clone();
            wal_path.push(format!("{}.wal", name));

            let wal_file = File::options()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(&wal_path)
                .with_context(|| format!("failed to open/create wal file({wal_path:?})"))?;

            let database = Database::new(data_file, wal_file);

            self.databases.push(database);
        }

        Ok(())
    }

    pub fn new_database<'a>(&'a mut self, name: &str) -> Result<&'a Database> {
        let mut data_path = self.path.clone();
        data_path.push(format!("{}.data", name));

        let data_file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&data_path)
            .with_context(|| format!("failed to open/create data file({data_path:?})"))?;

        let mut wal_path = self.path.clone();
        wal_path.push(format!("{}.wal", name));

        let wal_file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&wal_path)
            .with_context(|| format!("failed to open/create wal file({wal_path:?})"))?;

        let database = Database::new(data_file, wal_file);

        self.databases.push(database);
        self.metadata.database_names.push(name.to_string());
        self.metadata
            .save()
            .context("failure to save metadata after add database")?;

        Ok(self.databases.last().expect("Missing pushed database"))
    }
}
