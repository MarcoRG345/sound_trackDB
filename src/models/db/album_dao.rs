use rusqlite::{params, Connection, Result};
use crate::models::albums;
pub struct album_dao{
	connection: Connection,
}
impl Album_dao{
	pub fn new() -> Result<Self>{
		let path = "src/models/music.db";
		let conn = Connection::open(path)?;
		Connection{
			conn,
		}
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
	
	pub fn add_album(&self, album: &Album) -> Result<()>{
		self.connection.execute(
			"INSERT INTO albums (id_albums, path, name, year) VALUES (?1, ?2, ?3)",
			(album.get_id(), album.get_path(), album.get_name(), album.get_year()),
		)?;
		Ok(())
	}
	pub fn get_albums(&self) -> Result<()>{
		let mut stmut = self.connection.prepare("SELECT id_albums, path, name, year FROM albums");
		let albums_iter = stmut.query_map([], |row| {
			Ok(Albums {
				id_album: row.get(0)?,
				path: row.get(1)?,
				name: row.get(2)?,
				year: row.get(3)?,
			})
		})?;
		Ok(())
	}
}
#[cfg(test)]
mod tests{
	use super::*;
	
	#[test]
	fn test_connection(){
		let album_dao = Album_dao::new();
		let album = Albums::new(1, "not path".to_string(), "name", 2004);
		album_dao.create_album_table();
		album_dao.add_album(&album);
		
	}
}
