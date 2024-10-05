use std::sync::{Arc, Mutex};
use rusqlite::{params, Connection, Result};
use crate::models::performer::Performer;
use crate::models::media_attributes::MediaAttributes;
use crate::models::song::Song;
use crate::models::types::Types;
use crate::models::albums::Albums;


pub struct SongDao{
	connection: Arc<Mutex<Connection>>,
}

impl SongDao {
	pub fn new(connection: Arc<Mutex<Connection>>) -> Self{
		SongDao { connection, }
	}
	pub fn create_song_table(&self) -> Result<()>{
		let connection_key = self.connection.lock().unwrap();
		connection_key.execute(
			"CREATE TABLE IF NOT EXISTS rolas (
				 id_rola       INTEGER PRIMARY KEY AUTOINCREMENT,
    			 id_performer  INTEGER,
   				 id_albums      INTEGER,
   				 path          TEXT,
   				 title         TEXT,
   				 track         INTEGER,
   				 year          INTEGER,
   				 genre         TEXT,
   				 FOREIGN KEY   (id_performer) REFERENCES performers(id_performer)
    			 FOREIGN KEY   (id_albums) REFERENCES albums(id_albums)
			)"
			,(),)?;
		Ok(())
	}
	pub fn add_song(&self, song: &Song) -> Result<i64>{
		let connection_key = self.connection.lock().unwrap();
		connection_key.execute(
			"INSERT INTO rolas (id_performer, id_albums, path, title, track, year, genre) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
			(song.get_performer().get_id_perf(), song.get_album().get_id(), song.get_access().get_path(), song.get_tittle(), song.get_track(),
			song.get_access().get_year(), song.get_genre()),
		)?;
		Ok(connection_key.last_insert_rowid())
	}
	pub fn get_all_songs(&self) -> Result<Vec<Song>>{
		let connection_key = self.connection.lock().unwrap();
		let mut stmut = connection_key.prepare(
			"SELECT *, 
				albums.name AS albums_name, 
				albums.path AS albums_path, 
				albums.year AS albums_year, 
				performers.name AS performers_name, 
				types.description AS  types_description FROM rolas
			JOIN albums ON rolas.id_albums = albums.id_albums
			JOIN performers ON rolas.id_performer = performers.id_performer
			JOIN types ON performers.id_type = types.id_type"
		)?;
		let song_rows = stmut.query_map([], |row| {
			let types = Types::new(row.get::<_, String>("types_description")?);
			let performer = Performer::new(row.get::<_, String>("performers_name")?, types);
			let album = Albums::new(row.get::<_, String>("albums_path")?, row.get::<_, String>("albums_name")?, row.get::<_, u32>("albums_year")?);
			let access = MediaAttributes::new(row.get::<_, String>("path")?, row.get::<_, u32>("year")?);
			let song = Song::new(performer, album, row.get::<_, String>("title")?, access, row.get::<_, i32>("track")?, row.get::<_, String>("genre")?);
			Ok((song))
		})?;

		let mut songs_iter = Vec::new();
		for song in song_rows {
			if let Ok(song_) = song{
				songs_iter.push(song_);	
			}
		}
		Ok(songs_iter)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;
	use std::io::Error;
	use crate::models::db::db_connection::DBConnection;
	use crate::models::db::performer_dao::PerformerDao;
	use crate::models::db::types_dao::TypesDao;
	use crate::models::db::album_dao::Album_dao;

	fn delete_test_file(file_path: &str) -> Result<(), Error> {
    	fs::remove_file(file_path)?;
    	Ok(())
	}
	
	#[test]
	fn test_conn_song() -> Result<()>{
		let db = DBConnection::establish("src/models/db/test_conn_song.db")?;
		let types = Types::new("Group".to_string());
		let mut album = Albums::new("not path".to_string(), "DeadLines".to_string(), 2017);
		
		
		let types_dao = TypesDao::new(db.get_connection());
		types_dao.create_types_table();

		let mut  performer = Performer::new("Mr.kitty".to_string(), types);
		let performer_dao = PerformerDao::new(db.get_connection());
		performer_dao.create_perform_table();
		performer_dao.add_perform(&performer);
		performer.set_id(1);
		
		let album_dao = Album_dao::new(db.get_connection());
		album_dao.create_album_table();
		album_dao.add_album(&album);
		album.set_id_album(1);
		
		let access = MediaAttributes::new("not path".to_string(), 2019);
		let song = Song::new(performer, album, "After Dark".to_string(), access, 3123, "indie-electro".to_string());
		let song_dao = SongDao::new(db.get_connection());
		song_dao.create_song_table();
		song_dao.add_song(&song);

		let songs_iter = song_dao.get_all_songs()?;
		assert_eq!(songs_iter.len(), 1);
		
		delete_test_file("src/models/db/test_conn_song.db");
		Ok(())
	}
}
