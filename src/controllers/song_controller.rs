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
	pub fn get_last_song(&self) -> Song{
		match self.songDao.get_last_song(){
			Ok(song) => song,
			_=> todo!(),
		}
	}
	pub fn get_query_with(&self, condition_value: &String)-> Option<Vec<Song>>{
		println!("entra aqui?");
		if self.songDao.return_query(condition_value).unwrap().is_empty(){
			println!("esta vacio");
			return None;
		}
		Some(self.songDao.return_query(condition_value).unwrap())
	}
}
