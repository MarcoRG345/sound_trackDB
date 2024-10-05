use rusqlite::{params, Connection, Result};
use crate::models::albums::Albums;
use std::any::type_name;
use std::sync::{Arc, Mutex};

pub struct Album_dao{
	connection: Arc<Mutex<Connection>>,
}


impl Album_dao{
	pub fn new(connection: Arc<Mutex<Connection>>) -> Self{
		Album_dao{
			 connection,
		}
	}
	
	pub fn create_album_table(&self) -> Result<()>{
		self.connection.lock().unwrap().execute(
			"CREATE TABLE IF NOT EXISTS albums (
				id_albums INTEGER PRIMARY KEY AUTOINCREMENT,
				path TEXT,
				name TEXT,
				year INTEGER
			)",
			(),
		)?;
		Ok(())
	}
	
	pub fn add_album(&self, album: &Albums) -> Result<i64>{
		let connection_key = self.connection.lock().unwrap();
		connection_key.execute(
			"INSERT INTO albums (path, name, year) VALUES (?1, ?2, ?3)",
			(album.get_path(), album.get_name(), album.get_year()),
		)?;
		Ok(connection_key.last_insert_rowid())
	}
	
	pub fn get_albums(&self) -> Result<Vec<Albums>>{
		let connection_key = self.connection.lock().unwrap();
		let mut stmut = connection_key.prepare("SELECT id_albums, path, name, year FROM albums")?;
		let albums_rows = stmut.query_map([], |row| {
			let album_id = row.get(0)?;
			let mut album = Albums::new(row.get(1)?, row.get(2)?, row.get(3)?);
			album.set_id_album(album_id);
			Ok(album)
		})?;
		let mut album_iter = Vec::new();
		for album in albums_rows{
			if let Ok(album_result) = album {
				album_iter.push(album_result);
			}
		}
		Ok(album_iter)
	}
}

#[cfg(test)]
pub mod tests{
	use super::*;
	use std::fs;
	use std::io::Error;
	use crate::models::db::db_connection::DBConnection;
	
	fn delete_test_file(file_path: &str) -> Result<(), Error> {
    	fs::remove_file(file_path)?;
    	Ok(())
	}
	
	#[test]
	fn test_connection() -> Result<()>{
		let connection = DBConnection::establish("src/models/db/test_conn_album.db")?;
		let album_dao = Album_dao::new(connection.get_connection());
		let album = Albums::new("not path".to_string(), "name".to_string(), 2004);
		album_dao.create_album_table();
		album_dao.add_album(&album);
		let album_iter = album_dao.get_albums().expect("is empty");
		assert_eq!(album_iter.is_empty(), false);
		for i in 0..1000{
			let album = Albums::new("not path".to_string(), "name".to_string(), ((i + 100)).try_into().unwrap());
			album_dao.create_album_table();
			album_dao.add_album(&album);
		}
		let new_alb_iter = album_dao.get_albums().expect("is empty");
		assert_eq!(new_alb_iter.len(), 1001);
		delete_test_file("src/models/db/test_conn_album.db");
		Ok(())
	}
}
