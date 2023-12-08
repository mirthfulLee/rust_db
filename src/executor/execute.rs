use super::super::sql_analyzer::types::*;
use super::super::storage::StoreUtil;
use super::types::*;
use std::fmt::Display;
use tabled::settings::style::{HorizontalLine, VerticalLine};
use tabled::{builder::Builder, settings::Style};

impl SqlTable {
    /// used to create a new empty table
    pub fn new(columns: ColumnInfo) -> SqlTable {
        Self {
            columns,
            rows: Vec::new(),
        }
    }
}

impl Executable for CreateStatement {
    // fn check_and_execute(self, storage_util:StoreUtil) -> Result<ExecuteResponse, QueryExecutionError> {
    fn check_and_execute(self) -> Result<ExecuteResponse, QueryExecutionError> {
        // pub enum StoreUtil {
        //     /// The persistent data table is in csv format
        //     Csv(String),
        //     /// The persistent data table is in json format
        //     Json(String),
        // }
    
        // pub struct CreateStatement {
        //     pub table: String,
        //     pub columns: Vec<Column>,
        // }

        // pub enum ExecuteResponse {
        //     Message(String),
        //     Count(usize),
        //     View(Box<SqlTable>)
        // }

        // pub struct Column {
        //     pub name: String,
        //     pub type_info: SqlType,
        // }

        // pub enum SqlType {
        //     // these are basic for now. Will add more + size max later on
        //     String,
        //     Int,
        // }
        
        todo!()
    }
}

impl Executable for DropStatement {
    // fn check_and_execute(self, storage_util:StoreUtil) -> Result<ExecuteResponse, QueryExecutionError> {
    fn check_and_execute(self) -> Result<ExecuteResponse, QueryExecutionError> {
        todo!()
    }
}

impl Executable for InsertStatement {
    fn check_and_execute(self) -> Result<ExecuteResponse, QueryExecutionError> {
    // fn check_and_execute(self, storage_util:StoreUtil) -> Result<ExecuteResponse, QueryExecutionError> {
        todo!()
    }
}

impl Executable for DeleteStatement {
    fn check_and_execute(self) -> Result<ExecuteResponse, QueryExecutionError> {
    // fn check_and_execute(self, storage_util:StoreUtil) -> Result<ExecuteResponse, QueryExecutionError> {
        todo!()
    }
}

impl Executable for SelectStatement {
    fn check_and_execute(self) -> Result<ExecuteResponse, QueryExecutionError> {
    // fn check_and_execute(self, storage_util:StoreUtil) -> Result<ExecuteResponse, QueryExecutionError> {
        todo!()
    }
}

impl Executable for UpdateStatement {
    fn check_and_execute(self) -> Result<ExecuteResponse, QueryExecutionError> {
    // fn check_and_execute(self, storage_util:StoreUtil) -> Result<ExecuteResponse, QueryExecutionError> {
        todo!()
    }
}

impl Executable for SqlQuery {
    fn check_and_execute(self) -> Result<ExecuteResponse, QueryExecutionError> {
    // fn check_and_execute(self, storage_util:StoreUtil) -> Result<ExecuteResponse, QueryExecutionError> {
        match self {
            // SqlQuery::Create(stmt) => stmt.check_and_execute(storage_util),
            // SqlQuery::Drop(stmt) => stmt.check_and_execute(storage_util),
            // SqlQuery::Insert(stmt) => stmt.check_and_execute(storage_util),
            // SqlQuery::Delete(stmt) => stmt.check_and_execute(storage_util),
            // SqlQuery::Update(stmt) => stmt.check_and_execute(storage_util),
            // SqlQuery::Select(stmt) => stmt.check_and_execute(storage_util),

            SqlQuery::Create(stmt) => stmt.check_and_execute(),
            SqlQuery::Drop(stmt) => stmt.check_and_execute(),
            SqlQuery::Insert(stmt) => stmt.check_and_execute(),
            SqlQuery::Delete(stmt) => stmt.check_and_execute(),
            SqlQuery::Update(stmt) => stmt.check_and_execute(),
            SqlQuery::Select(stmt) => stmt.check_and_execute(),
        }
    }
}

impl Into<String> for SqlValue {
    fn into(self) -> String {
        match self {
            SqlValue::String(s) => s,
            SqlValue::Int(i) => i.to_string(),
            _ => String::from("Unknow"),
        }
    }
}

impl Display for ExecuteResponse {
    // Pretty print select result
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecuteResponse::Message(s) => s.fmt(f),
            ExecuteResponse::Count(cnt) => cnt.fmt(f),
            ExecuteResponse::View(table) => {
                let mut builder = Builder::default();
                for row in table.rows.iter() {
                    builder.push_record(row.values.clone());
                }
                let header = table
                    .columns
                    .iter()
                    .map(|col| col.name.clone())
                    .collect::<Vec<String>>();
                builder.set_header(header);
                let mut table = builder.build();
                let style = Style::modern()
                    .remove_horizontals()
                    .remove_verticals()
                    .horizontals([HorizontalLine::new(1, Style::modern().get_horizontal())
                        .main(Some('═'))
                        .intersection(None)])
                    .verticals([VerticalLine::new(1, Style::modern().get_vertical())]);
                table.with(style);
                table.fmt(f)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_table() {
        let res = ExecuteResponse::View(Box::new(SqlTable {
            columns: vec![
                Column {
                    name: "id".into(),
                    type_info: SqlType::Int,
                },
                Column {
                    name: "des".into(),
                    type_info: SqlType::String,
                },
            ],
            rows: vec![
                RowValue {
                    values: vec![SqlValue::Int(1), SqlValue::String("aabbccdd".into())],
                },
                RowValue {
                    values: vec![SqlValue::Int(123), SqlValue::String("aabbcc".into())],
                },
                RowValue {
                    values: vec![
                        SqlValue::Int(11),
                        SqlValue::String("aabbccddaabbccdd".into()),
                    ],
                },
                RowValue {
                    values: vec![SqlValue::Int(2141), SqlValue::String("aabbccdd".into())],
                },
            ],
        }));
        println!("{}", res);
    }
}
