use std::cell::LazyCell;
use std::sync::Arc;

use self::instance::memory::heap::InMemory;
use self::instance::memory::RwPool;

mod database;
mod instance;
