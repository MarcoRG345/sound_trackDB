use rusqlite::{params, Connection, Result};
use crate::models::performer::Performer;
use crate::models::types::Types;
use crate::models::db::types_dao::TypesDao;
use std::sync::{Arc, Mutex};

pub struct PerformerDao{
	connection: Arc<Mutex<Connection>>,
}
impl PerformerDao{
	pub fn new(connection: Arc<Mutex<Connection>>) -> Self{
		PerformerDao{ connection, }
	}
	pub fn create_perform_table(&self) -> Result<()>{
		let connection_key = self.connection.lock().unwrap();		
		connection_key.execute(
			"CREATE TABLE IF NOT EXISTS performers (
				id_performer INTEGER PRIMARY KEY AUTOINCREMENT,
				id_type INTEGER,
				name TEXT,
				FOREIGN KEY (id_type) REFERENCES types(id_type)
			)",(),)?;
		Ok(())
	}
	pub fn add_perform(&self, performer: &Performer) -> Result<()>{
		self.connection.lock().unwrap().execute(
			"INSERT INTO performers (id_type, name) VALUES (?1 ,?2)", (performer.get_type().get_id_type(), performer.get_name()),
		)?;
		Ok(())
	}
	
	pub fn get_performers(&self) -> Result<Vec<Performer>>{
		let connection_key = self.connection.lock().unwrap();
		let mut stmut = connection_key.prepare(
			"SELECT name, description FROM performers JOIN types ON 
			performers.id_type = types.id_type
			")?;
		let perform_rows = stmut.query_map([], |row| {
			let types = Types::new(row.get(1)?);
			let performer = Performer::new(row.get(0)?, types);
			Ok(performer)
		})?;
		let mut perform_iter = Vec::new();
		
		for performer in perform_rows{
			if let Ok(performer_res) = performer{	
				perform_iter.push(performer_res);
			} 
		}
		Ok(perform_iter)
	}
	
}

#[cfg(test)]
mod tests{
	use super::*;
	use std::fs;
	use std::io::Error;
	use crate::models::db::db_connection::DBConnection;
	
	
	fn delete_test_file(file_path: &str) -> Result<(), Error> {
    	fs::remove_file(file_path)?;
    	Ok(())
	}
	
	#[test]
	fn test_conn_performer() -> Result<()>{
		let connection = DBConnection::establish("src/models/db/test_performer.db")?;
		
		let types_dao = TypesDao::new(connection.get_connection());
		types_dao.create_types_table();
		
		let performer_dao = PerformerDao::new(connection.get_connection());
		performer_dao.create_perform_table();
		
		for i in 0..1000{
			let types_person = Types::new("Person".to_string());
			let types_group = Types::new("Group".to_string());
			let performer = Performer::new("Mr.kitty".to_string(), types_person);
			let performer_group = Performer::new("Zurdok".to_string(), types_group);
			performer_dao.add_perform(&performer);
			performer_dao.add_perform(&performer_group);
		}
		
		let performers_iter = performer_dao.get_performers()?;
		assert_eq!(performers_iter.len(), 2000);
		
		delete_test_file("src/models/db/test_performer.db");
		Ok(())
		
	}
}



		
		
			

