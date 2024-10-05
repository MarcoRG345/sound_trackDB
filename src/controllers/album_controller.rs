use crate::models::albums::Albums;
use rusqlite::{params, Connection, Result};
use crate::models::db::album_dao::Album_dao;
use std::sync::{Arc, Mutex};

pub struct AlbumController{
	albumDao: Album_dao,
}

impl AlbumController{
	pub fn new(connection: Arc<Mutex<Connection>>) -> Self{
		let albumDao = Album_dao::new(connection);
		AlbumController{
			albumDao,
		}
	}
	pub fn add_album(&self, album: &Albums) -> i64{
		self.albumDao.create_album_table();
		let id: i64 = match self.albumDao.add_album(album){
			Ok(id_) => id_,
			_ => 0,
		};
		id
	}
}

