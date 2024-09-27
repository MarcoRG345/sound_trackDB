use crate::models::types::Types;
use rusqlite::{params, Connection, Result};
pub struct TypesDao{
	connection: Connection,
}
impl TypesDao{	
	pub fn new () -> Result<Self>{
		let connection = Connection::open("src/models/db/music.db")?;
		Ok(TypesDao { connection, })
	}
	pub fn create_types_table(&self) -> Result<()>{
		self.connection.execute(
			"CREATE TABLE IF NOT EXISTS types (
				id_types INTEGER PRIMARY KEY,
				decription TEXT
			)",
		(),)?;
		self.add_values();
		Ok(())
	}
	fn add_values(&self) -> Result<()>{
		self.connection.execute_batch(
			"INSERT INTO types VALUES (0, 'Person');
			 INSERT INTO types VALUES (1, 'Group');
			 INSERT INTO types VALUES (3, 'Unknown');",
		)?;
		Ok(())
	}
	pub fn set_connection(&mut self, connection: Connection){
		self.connection = connection;
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	use std::fs;
	use std::io::Error;
	
	#[test]
	fn test_conn_types() -> Result<()>{
		let mut types_dao = TypesDao::new()?;
		let conn = Connection::open("src/models/db/test_data_types.db")?;
		types_dao.set_connection(conn);
		types_dao.create_types_table();
		let types_iter = get_all_types()?;
		assert_eq!(types_iter.len(), 3);
		for types in types_iter{
			println!("-------------------------------");
			println!("ID: {}", types.get_id_type());
			println!("description: {}", types.get_description());
			println!("-------------------------------");
		}
		delete_test_file("src/models/db/test_data_types.db");
		Ok(())
	}
	fn delete_test_file(file_path: &str) -> Result<(), Error> {
    	fs::remove_file(file_path)?;
    	Ok(())
	}
	fn get_all_types() -> Result<Vec<Types>>{
		let conn = Connection::open("src/models/db/test_data_types.db").expect("failed");
		let mut stmut = conn.prepare("SELECT * FROM types")?;
		let type_row = stmut.query_map([], |row| {
			let mut types = Types::new(row.get(1)?);
			types.set_id_type(row.get(0)?);
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
