use yawdb::storage::Storage;

#[test]
fn test_storage() {
    let mut storage = Storage::new("test.db");
    storage.write(0, b"hello");
    let data = storage.read(0, 5);
    assert_eq!(&data, b"hello");
}
