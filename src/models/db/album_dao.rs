use rusqlite::{params, Connection, Result};
use crate::models::albums::Albums;
use std::any::type_name;

pub struct Album_dao{
	connection: Connection,
}


impl Album_dao{
	pub fn new() -> Result<Self>{
		let connection = Connection::open("src/models/db/music.db").expect("Error opening database");
		Ok(Album_dao{
			 connection,
		})
	}
	
	pub fn create_album_table(&self) -> Result<()>{
		self.connection.execute(
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
	
	pub fn add_album(&self, album: &Albums) -> Result<(), >{
		self.connection.execute(
			"INSERT INTO albums (path, name, year) VALUES (?1, ?2, ?3)",
			(album.get_path(), album.get_name(), album.get_year()),
		)?;
		Ok(())
	}
	
	pub fn get_albums(&self) -> Result<Vec<Albums>>{
		let mut stmut = self.connection.prepare("SELECT id_albums, path, name, year FROM albums")?;
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

	
	fn delete_test_file(file_path: &str) -> Result<(), Error> {
    	fs::remove_file(file_path)?;
    	Ok(())
	}
	
	#[test]
	fn test_connection() -> Result<()>{
		let album_dao = Album_dao::new()?;
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
		delete_test_file("src/models/db/music.db");
		Ok(())
	}
}
