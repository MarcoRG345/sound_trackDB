use crate::models::db::song_dao::SongDao;
use crate::models::song::Song;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};

pub struct SongController{
	songDao: SongDao,
}
impl SongController{
	pub fn new(connection: Arc<Mutex<Connection>>) -> Self{
		let songDao = SongDao::new(connection);
		SongController{
			songDao,
		}
	}
	pub fn add_song(&self, song: &Song){
		self.songDao.create_song_table();
		self.songDao.add_song(song);
	}
}
