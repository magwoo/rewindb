use flemish_kv::Database;

fn main() {
    let database = Database::open("./test.data").unwrap();

    for i in 0..60000 {
        database
            .insert(
                &format!("{:x}", i * 1234567_u128),
                format!("{:x}", i * 7654321_u128).as_bytes(),
            )
            .unwrap();
    }

    println!(
        "result: {:?}",
        database.get(&format!("{:x}", 3 * 1234567_u128))
    );
}
