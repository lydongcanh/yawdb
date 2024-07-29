use crate::storage::Storage;
use sqlparser::ast::{ObjectName, Query, SelectItem, SetExpr, Statement, TableWithJoins, Assignment, Expr, TableFactor, Ident};

pub struct Executor {
    storage: Storage,
}

impl Executor {
    pub fn new(storage: Storage) -> Self {
        Executor { storage }
    }

    pub fn execute(&mut self, statement: Statement) {
        match statement {
            Statement::Query(query) => self.execute_select(*query),
            Statement::Insert { table_name, columns, source, .. } => self.execute_insert(table_name, columns, source),
            Statement::Update { table, assignments, selection, .. } => self.execute_update(table, assignments, selection),
            Statement::Delete { table_name, selection, .. } => self.execute_delete(table_name, selection),
            _ => println!("Unsupported statement"),
        }
    }

    fn execute_select(&self, query: Query) {
        if let SetExpr::Select(select) = query.body {
            for item in select.projection {
                match item {
                    SelectItem::UnnamedExpr(expr) => println!("Select expression: {:?}", expr),
                    _ => println!("Other select item"),
                }
            }
        }
    }

    fn execute_insert(&mut self, table_name: ObjectName, columns: Vec<Ident>, source: Box<Query>) {
        let table_name = table_name.to_string(); // Convert ObjectName to String
        println!("Insert into table: {}", table_name);
        // Implementation for inserting data into the storage
    }

    fn execute_update(&mut self, table: TableWithJoins, assignments: Vec<Assignment>, selection: Option<Expr>) {
        // Handle the table name from the TableWithJoins
        if let TableFactor::Table { name, .. } = table.relation {
            let table_name = name.to_string(); // Convert ObjectName to String
            println!("Update table: {}", table_name);
            // Handle assignments and selection
        } else {
            println!("Unsupported table factor in UPDATE");
        }
    }

    fn execute_delete(&mut self, table_name: ObjectName, selection: Option<Expr>) {
        let table_name = table_name.to_string(); // Convert ObjectName to String
        println!("Delete from table: {}", table_name);
        // Implementation for deleting data from the storage
    }
}
