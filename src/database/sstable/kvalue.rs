#[derive(Debug)]
pub struct KValue {
    checksum: u32,
    key: Vec<u8>,
    value: Vec<u8>,
}
