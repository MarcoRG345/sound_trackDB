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
				id_albums INTEGER PRIMARY KEY,
				path TEXT,
				name TEXT,
				year INTEGER
			)",
			(),
		)?;
		Ok(())
	}
	
	pub fn add_album(&self, album: &Albums) -> Result<()>{
		self.connection.execute(
			"INSERT INTO albums (id_albums, path, name, year) VALUES (?1, ?2, ?3, ?4)",
			(album.get_id(), album.get_path(), album.get_name(), album.get_year()),
		)?;
		Ok(())
	}
	pub fn get_albums(&self) -> Result<Vec<Albums>>{
		let mut stmut = self.connection.prepare("SELECT id_albums, path, name, year FROM albums")?;
		let albums_rows = stmut.query_map([], |row| {
			Ok(Albums::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
		})?;
		let mut album_iter = Vec::new();
		for album in albums_rows{
			let album = album.unwrap();
			album_iter.push(album);
		}
		Ok(album_iter)
	}
}

#[cfg(test)]
pub mod tests{
	use super::*;
	 use std::env;
	
	#[test]
	fn test_connection() -> Result<()>{
		let album_dao = Album_dao::new()?;
		let album = Albums::new(1, "not path".to_string(), "name".to_string(), 2004);
		album_dao.create_album_table();
		album_dao.add_album(&album);
		let album_iter = album_dao.get_albums().expect("is empty");
		assert_eq!(album_iter.is_empty(), false);
		for i in 0..1000{
			let album = Albums::new(i, "not path".to_string(), "name".to_string(), ((i + 100)).try_into().unwrap());
			album_dao.create_album_table();
			album_dao.add_album(&album);
		}
		let new_alb_iter = album_dao.get_albums().expect("is empty");
		assert_eq!(new_alb_iter.len(), 1000);
		let current_dir = env::current_dir().unwrap();
    	println!("Current directory: {:?}", current_dir); 
		Ok(())
	}
}
