#![allow(unused)]
use anyhow::{bail, Context, Result};
use std::collections::{HashMap, HashSet};
use std::fs::{read_dir, DirBuilder, File};
use std::io::{prelude::*, Cursor};
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Instance {
    databases: Vec<Database>,
    metadata: InstanceMetadata,
}

impl Instance {
    pub fn new(path: &str) -> Result<Self> {
        let dir_path = PathBuf::from(path);

        DirBuilder::new()
            .recursive(true)
            .create(&dir_path)
            .with_context(|| format!("Error while open/create dir {dir_path:?}"))?;

        let mut metadata_path = dir_path.clone();
        metadata_path.push("database.metadata");

        let metadata =
            InstanceMetadata::new(&metadata_path).context("Error while load instance metadata")?;

        Ok(Self {
            databases: Vec::new(),
            metadata,
        })
    }

    // pub fn open(&mut self, name: &str) -> Result<Database> {
    //     let file =

    // }
}

#[derive(Debug)]
struct InstanceMetadata {
    f: File,
    version: InstanceVersion,
    database_names: Vec<String>,
}

impl InstanceMetadata {
    pub fn new(path: &Path) -> Result<Self> {
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .with_context(|| format!("Error while open metadata file {path:?}"))?;

        let version =
            InstanceVersion::from_reader(&mut file).context("Reading version from file error")?;

        let mut len_buf = [0; 4];
        let readed = file
            .read(&mut len_buf)
            .context("Read database names len buf error")?;

        let database_names = if readed != len_buf.len() {
            Vec::new()
        } else {
            let len = u32::from_le_bytes(len_buf);

            let mut buf = vec![0; len as usize];
            file.read_exact(&mut buf)
                .context("Read database names buf error");

            let mut buf = Cursor::new(buf);
            let mut names = Vec::new();

            loop {
                let mut len_buf = [0; 2];
                let readed = buf
                    .read(&mut len_buf)
                    .expect("Database names read name len buf error");

                if readed != len_buf.len() {
                    break;
                }

                let len = u16::from_le_bytes(len_buf);

                let mut name_buf = vec![0; len as usize];
                buf.read_exact(&mut name_buf)
                    .context("Read database name error")?;

                names.push(String::from_utf8_lossy(&name_buf).to_string());
            }

            names
        };

        if version != InstanceVersion::current() {
            bail!("Data has difference version!");
        }

        Ok(Self {
            f: file,
            version,
            database_names,
        })
    }

    pub fn save(&mut self) -> Result<()> {
        self.f
            .seek(std::io::SeekFrom::Start(0))
            .context("failed to seek")?;

        self.version
            .save_to_writer(&mut self.f)
            .context("failed to save version to metadata")?;

        if self.database_names.is_empty() {
            self.f.write_all(&0_u16.to_le_bytes())
        } else {
            let mut total_len = self.database_names.len() * 2;

            self.database_names
                .iter()
                .for_each(|n| total_len += n.bytes().len());

            self.f.write_all(&(total_len as u16).to_le_bytes())
        }
        .context("failed to write database names len")?;

        for name in self.database_names.iter() {
            self.f
                .write_all(name.as_bytes())
                .with_context(|| format!("failed to save database name({name})"))?;
        }

        self.f.flush().context("failed to flush metadata");

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct InstanceVersion {
    major: u16,
    minor: u16,
    patch: u16,
    pre: Option<String>,
}

impl InstanceVersion {
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut len_buf = [0; 1];
        let len = reader
            .read(&mut len_buf)
            .context("Error while read metadata version len buffer")?;

        if len == 0 {
            return Ok(Self::current());
        }

        let len = u8::from_le_bytes(len_buf);

        let mut buf = vec![0; (len as usize) + 6];
        reader
            .read_exact(&mut buf)
            .context("Error while read metadata buffer")?;

        let major = u16::from_le_bytes(buf[..2].try_into().expect("Missing major bytes"));
        let minor = u16::from_le_bytes(buf[2..4].try_into().expect("Missing minor bytes"));
        let patch = u16::from_le_bytes(buf[4..6].try_into().expect("Missing patch bytes"));

        let pre = if len == 0 {
            None
        } else {
            Some(String::from_utf8_lossy(&buf[6..]).to_string())
        };

        Ok(Self {
            major,
            minor,
            patch,
            pre,
        })
    }

    pub fn current() -> Self {
        Self {
            major: 0,
            minor: 0,
            patch: 0,
            pre: Some("alpha".to_string()),
        }
    }

    pub fn save_to_writer<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self.pre.as_ref() {
            Some(pre) => writer.write_all(&(pre.as_bytes().len() as u8).to_le_bytes()),
            None => writer.write_all(&0_u8.to_le_bytes()),
        }
        .context("failed to write pre version len")?;

        writer
            .write_all(&self.major.to_le_bytes())
            .context("failed to write major")?;

        writer
            .write_all(&self.minor.to_le_bytes())
            .context("failed to write minor")?;

        writer
            .write_all(&self.patch.to_le_bytes())
            .context("failed to write patch")?;

        if let Some(pre) = self.pre.as_ref() {
            writer
                .write_all(pre.as_bytes())
                .context("failed to write pre version")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Database {
    f: File,
    wal: File,
    memtable: SSTable,
    transactions: Vec<File>,
    index: HashMap<Vec<u8>, u64>,
    cache: Vec<SSTable>,
}

// impl Database {
//     pub fn open(f: File, wal: File) -> Result<Self> {
//         let file = FileOpe
//     }
// }

#[derive(Debug)]
struct SSTable {
    filter: TableFilter,
    value: Vec<KValue>,
}

#[derive(Debug)]
struct TableFilter(HashSet<u32>);

#[derive(Debug)]
struct KValue {
    checksum: u32,
    key: Vec<u8>,
    value: Vec<u8>,
}

#[derive(Debug)]
enum Action {
    Insert(String, String),
    Update(String, String),
    Remove(String),
    Get(String),
}

const USAGE: &str = r#"usage:
cargo run [file] [action] [key] [value?]

[file] - *
[action] - insert, update, remove, get
[key] - *
[value] - *

example:
cargo run insert 1 boris
"#;

fn main() {
    let mut instance = Instance::new("./data")
        .context("Create instance error")
        .unwrap();

    println!("instance: {:#?}", instance);

    instance
        .metadata
        .save()
        .context("Instance metadata save error")
        .unwrap();

    println!("writed!");

    // let args = std::env::args().collect::<Vec<_>>();

    // let dbpath = args.get(1).expect(USAGE);

    // let command = match args.get(2).expect(USAGE).as_str() {
    //     "insert" => Action::Insert(
    //         args.get(3).expect(USAGE).to_string(),
    //         args.get(4).expect(USAGE).to_string(),
    //     ),
    //     "update" => Action::Update(
    //         args.get(3).expect(USAGE).to_string(),
    //         args.get(4).expect(USAGE).to_string(),
    //     ),
    //     "remove" => Action::Remove(args.get(3).expect(USAGE).to_string()),
    //     "get" => Action::Get(args.get(3).expect(USAGE).to_string()),
    //     _ => panic!("{}", USAGE),
    // };
}
