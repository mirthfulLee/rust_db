use super::super::storage::StoreUtil;
use super::types::*;
use super::super::sql_analyzer::types::*;
use std::fmt::Display;
use tabled::settings::style::{HorizontalLine, VerticalLine};
use tabled::{builder::Builder, settings::Style};
use std::io;


fn compare_name(names_insert: &Vec<String>, columns: &Vec<Column>) -> Option<String> {
    let mut names_columns: Vec<String> = Vec::new();
    for column in columns {
        names_columns.push(column.name.clone());
    } 
    for name_insert in names_insert {
        if !names_columns.contains(name_insert) {
            return Some(name_insert.clone());
        }
    }
    None
}

fn get_newrol(names_insert:Vec<String>,columns:&Vec<Column>,value:RowValue) -> Result<RowValue, QueryExecutionError>{
    match compare_name(&names_insert,columns) {
        None => {
            // Create an iterator over the names_table
            let mut row_values:Vec<SqlValue> = Vec::new();
            // let name_columns: Vec<String> = columns.iter().map(|column: &Column| column.name.clone()).collect();
            for column in columns {
                let mut flag=0;
                if let Some(index) = names_insert.iter().position(|name_insert| name_insert == &column.name) {
                    // row_value.push(value.clone().values[index]);
                    let values: &Vec<SqlValue>=&value.values;
                    match &values[index] {
                        SqlValue::String(row_value) => {
                            row_values.push(SqlValue::String((row_value.clone())));
                        }
                        SqlValue::Int(row_value) => {
                            row_values.push(SqlValue::Int(*row_value));
                        }
                        SqlValue::Unknown =>{
                            row_values.push(SqlValue::Unknown);
                        }
                    }
                } else {
                    match column.type_info {
                        SqlType::String => {
                            row_values.push(SqlValue::String("NULL".to_string()))
                        },
                        SqlType::Int => {
                            row_values.push(SqlValue::Int(0))
                        },
                        SqlType::Unknown => {
                            row_values.push(SqlValue::Unknown)
                        },
                    }
                }
            }
            let rowvalues:RowValue=RowValue{values:row_values};
            Ok(rowvalues)
        }
        Some(name_insert) => {
            return Err(QueryExecutionError::ColumnDoesNotExist(name_insert))
        }
    }

}

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
    fn check_and_execute(self, storage_util:StoreUtil) -> Result<ExecuteResponse, QueryExecutionError> {
        let name = self.table.clone();
        let columns_infos = self.columns;
        let table=SqlTable::new(columns_infos);
        match storage_util.save(name.clone(),&table){
            Ok(()) => {
                Ok(ExecuteResponse::Message(format!("save {} successful", name)))
            }
            Err(_) => {
                Err(QueryExecutionError::TableSavefail(name))
            }, 
        }
    }
}

impl Executable for DropStatement {
    fn check_and_execute(self, storage_util:StoreUtil) -> Result<ExecuteResponse, QueryExecutionError> {
        let name = self.table.clone();
        match storage_util.delete(&name){
            Ok(()) => {
                Ok(ExecuteResponse::Message(format!("delete {} successful", name)))
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    Err(QueryExecutionError::TableNotFound(name))
                }
                _ => {
                    Err(QueryExecutionError::TableDeletefail(name))
                }
            }, 
        }
    }
}

impl Executable for InsertStatement {
    fn check_and_execute(self, storage_util:StoreUtil) -> Result<ExecuteResponse, QueryExecutionError> {
        let name = self.table.clone();
        match storage_util.load(name.clone()){
            Ok((table)) => {
                let columns: Vec<Column> = table.columns;
                let mut rows: Vec<RowValue> = table.rows;
                let name_columns: Vec<String> = columns.iter().map(|column: &Column| column.name.clone()).collect();
                match self.columns {
                    Some(name_insert) => {
                        match get_newrol(name_insert,&columns,self.values) {
                            Ok(rowvalue) => {
                                rows.push(rowvalue);
                                let new_table = SqlTable{columns,rows};
                                match storage_util.save(name.clone(), &new_table){
                                    Ok(()) => {
                                        Ok(ExecuteResponse::Message(format!("save {} successful", name)))
                                    }
                                    Err(_) => {
                                        Err(QueryExecutionError::TableSavefail(name))
                                    }, 
                                }
                            },
                            Err(err) => {
                                Err(err)
                            }
                        }
                    }
                    None => {
                            let rowvalue = self.values;
                            rows.push(rowvalue);
                            let new_table = SqlTable{columns,rows};
                            match storage_util.save(name.clone(), &new_table){
                                Ok(()) => {
                                    Ok(ExecuteResponse::Message(format!("save {} successful", name)))
                                }
                                Err(_) => {
                                    Err(QueryExecutionError::TableSavefail(name))
                                }, 
                            }
                        },      
                    }

                }
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    Err(QueryExecutionError::TableNotFound(name))
                }
                _ => {
                    Err(QueryExecutionError::TableOpenfail(name))
                }
            }, 
        }
    }
}

impl Executable for DeleteStatement {
    // fn check_and_execute(self) -> Result<ExecuteResponse, QueryExecutionError> 
    fn check_and_execute(self, storage_util:StoreUtil) -> Result<ExecuteResponse, QueryExecutionError> {
        let name = self.table.clone();
        match storage_util.load(name.clone()){
            Ok((table)) => {
                // pub struct DeleteStatement {
                //     pub table: String,
                //     pub constraints: Option<WhereConstraint>,
                // }
                match self.constraints {
                    Some(constraints) => {
                        todo!()
                    }
                    None => {
                        Err(QueryExecutionError::TableNotFound(name))
                    }
                }
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    Err(QueryExecutionError::TableNotFound(name))
                }
                _ => {
                    Err(QueryExecutionError::TableOpenfail(name))
                }
            }, 
        }
    }
}

impl Executable for SelectStatement {
    // fn check_and_execute(self) -> Result<ExecuteResponse, QueryExecutionError> 
    fn check_and_execute(self, storage_util:StoreUtil) -> Result<ExecuteResponse, QueryExecutionError> {
        todo!()
    }
}

impl Executable for UpdateStatement {
    // fn check_and_execute(self) -> Result<ExecuteResponse, QueryExecutionError> 
    fn check_and_execute(self, storage_util:StoreUtil) -> Result<ExecuteResponse, QueryExecutionError> {
        todo!()
    }
}

impl Executable for SqlQuery {
    fn check_and_execute(self, storage_util:StoreUtil) -> Result<ExecuteResponse, QueryExecutionError> {
        match self {
            SqlQuery::Create(stmt) => stmt.check_and_execute(storage_util),
            SqlQuery::Drop(stmt) => stmt.check_and_execute(storage_util),
            SqlQuery::Insert(stmt) => stmt.check_and_execute(storage_util),
            SqlQuery::Delete(stmt) => stmt.check_and_execute(storage_util),
            SqlQuery::Update(stmt) => stmt.check_and_execute(storage_util),
            SqlQuery::Select(stmt) => stmt.check_and_execute(storage_util),


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

