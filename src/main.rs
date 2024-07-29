use yawdb::storage::Storage;

fn main() {
    env_logger::init();
    let mut storage = Storage::new("test.db");
    
    // Example usage
    storage.write(0, b"hello");
    let data = storage.read(0, 5);
    println!("Read data: {:?}", String::from_utf8(data).unwrap());
}
