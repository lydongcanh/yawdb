use yawdb::storage::Storage;
use yawdb::parser::parse_sql;
use yawdb::executor::Executor;

fn main() {
    env_logger::init();
    let storage = Storage::new("test.db");
    let mut executor = Executor::new(storage);

    let sql = "SELECT * FROM test;";
    match parse_sql(sql) {
        Ok(statements) => {
            for statement in statements {
                executor.execute(statement);
            }
        }
        Err(e) => println!("Error parsing SQL: {:?}", e),
    }
}
