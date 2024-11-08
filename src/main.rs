#![allow(unused)]
use anyhow::{bail, Context, Result};
use std::collections::{HashMap, HashSet};
use std::fs::{read_dir, DirBuilder, File};
use std::io::{prelude::*, Cursor};
use std::path::{Path, PathBuf};

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
    let memory = Memory::new();

    let mut instance = Instance::new("./data")
        .context("Create instance error")
        .unwrap();

    println!("instance: {:#?}", instance);

    instance
        .metadata
        .save()
        .context("Instance metadata save error")
        .unwrap();

    instance
        .new_database("dev")
        .context("failure to create `dev` database")
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
