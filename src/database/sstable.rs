use self::filter::TableFilter;
use self::kvalue::KValue;

mod filter;
mod kvalue;

#[derive(Debug, Default)]
pub struct SSTable {
    filter: TableFilter,
    value: Vec<KValue>,
}

// impl SSTable {
//     pub fn new() -> Self {
//         Self { filter: , value:  }
//     }
// }
