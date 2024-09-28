use crate::models::types::Types;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};

pub struct TypesDao{
	connection: Arc<Mutex<Connection>>,
}
impl TypesDao{	
	pub fn new (connection: Arc<Mutex<Connection>>) -> Self{
		TypesDao { connection, }
	}
	pub fn create_types_table(&self) -> Result<()>{
		self.connection.lock().unwrap().execute(
			"CREATE TABLE IF NOT EXISTS types (
				id_type INTEGER PRIMARY KEY,
				description TEXT
			)",
		(),)?;
		self.add_values();
		Ok(())
	}
	fn add_values(&self) -> Result<()>{
		self.connection.lock().unwrap().execute_batch(
			"INSERT INTO types VALUES (0, 'Person');
			 INSERT INTO types VALUES (1, 'Group');
			 INSERT INTO types VALUES (3, 'Unknown');",
		)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	use std::fs;
	use std::io::Error;
	use crate::models::db::db_connection::DBConnection;
	
	#[test]
	fn test_conn_types() -> Result<()>{
		let connection = DBConnection::establish("src/models/db/test_data_types.db")?;
		let mut types_dao = TypesDao::new(connection.get_connection());
		types_dao.create_types_table();
		let types_iter = get_all_types(connection.get_connection())?;
		assert_eq!(types_iter.len(), 3);
		/*for types in types_iter{
			println!("-------------------------------");
			println!("ID: {}", types.get_id_type());
			println!("description: {}", types.get_description());
			println!("-------------------------------");
		}*/
		delete_test_file("src/models/db/test_data_types.db");
		Ok(())
	}
	fn delete_test_file(file_path: &str) -> Result<(), Error> {
    	fs::remove_file(file_path)?;
    	Ok(())
	}
	fn get_all_types(connection: Arc<Mutex<Connection>>) -> Result<Vec<Types>>{
		let write_conn = connection.lock().unwrap();
		let mut stmut = write_conn.prepare("SELECT * FROM types")?;
		let type_row = stmut.query_map([], |row| {
			let mut types = Types::new(row.get(1)?);
			Ok(types)
		})?;
		let mut types_iter = Vec::new();
		for types in type_row{
			if let Ok(type_) = types{
				types_iter.push(type_);
			}
		}
		Ok(types_iter)
	} 
}
