use super::executor::types::*;

pub enum StoreUtil {
    /// The persistent data table is in csv format
    Csv(String),
    /// The persistent data table is in json format
    Json(String),
}

impl StoreUtil {
    /// check whether the table exists
    fn exists(name: String) -> bool {
        todo!()
    }

    /// load table with table name
    fn load(name: String) -> SqlTable {
        todo!()
    }
    /// save table persistently
    fn save(table: SqlTable) {
        todo!()
    }
}