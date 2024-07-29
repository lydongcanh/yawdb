use yawdb::storage::Storage;
use yawdb::parser::parse_sql;
use yawdb::executor::Executor;

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

#[test]
fn test_sql_select() {
    let storage = Storage::new("test.db");
    let mut executor = Executor::new(storage);

    let sql = "SELECT * FROM test;";
    match parse_sql(sql) {
        Ok(statements) => {
            for statement in statements {
                executor.execute(statement);
            }
        }
        Err(e) => panic!("Error parsing SQL: {:?}", e),
    }
}

#[test]
fn test_sql_insert() {
    let storage = Storage::new("test.db");
    let mut executor = Executor::new(storage);

    let sql = "INSERT INTO test (id, name) VALUES (1, 'Alice');";
    match parse_sql(sql) {
        Ok(statements) => {
            for statement in statements {
                executor.execute(statement);
            }
        }
        Err(e) => panic!("Error parsing SQL: {:?}", e),
    }
}

#[test]
fn test_sql_update() {
    let storage = Storage::new("test.db");
    let mut executor = Executor::new(storage);

    let sql = "UPDATE test SET name = 'Bob' WHERE id = 1;";
    match parse_sql(sql) {
        Ok(statements) => {
            for statement in statements {
                executor.execute(statement);
            }
        }
        Err(e) => panic!("Error parsing SQL: {:?}", e),
    }
}

#[test]
fn test_sql_delete() {
    let storage = Storage::new("test.db");
    let mut executor = Executor::new(storage);

    let sql = "DELETE FROM test WHERE id = 1;";
    match parse_sql(sql) {
        Ok(statements) => {
            for statement in statements {
                executor.execute(statement);
            }
        }
        Err(e) => panic!("Error parsing SQL: {:?}", e),
    }
}

#[test]
fn test_sql_unsupported() {
    let storage = Storage::new("test.db");
    let mut executor = Executor::new(storage);

    let sql = "CREATE TABLE test (id INT, name TEXT);";
    match parse_sql(sql) {
        Ok(statements) => {
            for statement in statements {
                executor.execute(statement);
            }
        }
        Err(e) => panic!("Error parsing SQL: {:?}", e),
    }
}