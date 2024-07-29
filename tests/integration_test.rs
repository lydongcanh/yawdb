use yawdb::storage::Storage;

#[test]
fn test_storage_write_read() {
    let mut storage = Storage::new("test.db");
    storage.write(0, b"hello");
    let data = storage.read(0, 5);
    assert_eq!(&data, b"hello");
}

#[test]
fn test_storage_append() {
    let mut storage = Storage::new("test.db");
    storage.append(b" world");
    let data = storage.read(5, 6);
    assert_eq!(&data, b" world");
}
