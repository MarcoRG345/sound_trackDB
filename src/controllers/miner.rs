use id3::{Tag, TagLike, Content};
use crate::models::db::db_connection::DBConnection;
use crate::models::performer::Performer;
use crate::models::media_attributes::MediaAttributes;
use crate::models::song::Song;
use crate::models::types::Types;
use crate::models::albums::Albums;
use std::fs;
use crate::controllers::performer_controller::PerformerController;
use crate::controllers::album_controller::AlbumController;
use crate::controllers::song_controller::SongController;
use crate::controllers::types_controller::TypeController;

use std::collections::HashMap;

pub struct Miner{
	db_connection: DBConnection,
	last_songs: Vec<Song>,
	metadata: HashMap<String, String>,
}

impl Miner{
	pub fn new(db_connection: DBConnection) -> Self{
		let mut metadata = HashMap::new();
		let mut last_songs = Vec::new();
		Miner{
			db_connection,
			last_songs,
			metadata,
		}
	}
	
	pub fn read_id3(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>>{
		let file_tags = Tag::read_from_path(path)?;
		for frame in file_tags.frames(){
			match frame.id(){
				"TPE1" => {
					self.metadata.insert("artist".to_string(), frame.content().to_string());
				},
				"TIT2" => {
					self.metadata.insert("title".to_string(), frame.content().to_string());
				},
				"TALB" => {
					self.metadata.insert("album".to_string(), frame.content().to_string());
				},
				"TDRC" => {
					self.metadata.insert("year".to_string(), frame.content().to_string());
				},
				"TRCK" => {
					self.metadata.insert("track".to_string(), frame.content().to_string());
				},
				"TCON" => {
					self.metadata.insert("genre".to_string(), frame.content().to_string());
				},
				_ => {print!("");},
			}
		}
		//dialogo para pregutnar si es persona o grupo
		self.insert_database(path);
		Ok(())	
	}

	pub fn insert_database(&mut self, path: &str){
		let song_controller = SongController::new(self.db_connection.get_connection());
		let album_controller = AlbumController::new(self.db_connection.get_connection());
		let performer_controller = PerformerController::new(self.db_connection.get_connection());
		
		let type_controller = TypeController::new(self.db_connection.get_connection());
		//show dialog to ask for user.
		let types = Types::new("Person".to_string());
		type_controller.create_types();
		
		let mut album = Albums::new(path.to_string(),
								self.metadata.get("album").unwrap_or(&"Unknown".to_string()).to_string(),
								self.metadata.get("year").map(|s| s.parse::<u32>().unwrap_or(0)).unwrap_or(0));
		let id_album = album_controller.add_album(&album);
		album.set_id_album(id_album);
		
		let mut performer = Performer::new(self.metadata.get("artist").unwrap_or(&"Unknown".to_string()).to_string(), types);	
		let id_perform = performer_controller.add_perform(&performer);
		performer.set_id(id_perform);
		
		let access =  MediaAttributes::new(path.to_string(), self.metadata.get("year").map(|s| s.parse::<u32>().unwrap_or(0)).unwrap_or(0));
		let song = Song::new(performer, album, self.metadata.get("title").unwrap_or(&"Unknown".to_string()).to_string(),
							 access, 3, self.metadata.get("genre").unwrap_or(&"Unknown".to_string()).to_string());
		song_controller.add_song(&song);
		self.last_songs.push(song_controller.get_last_song());
	}
	
	pub fn read_directory(&mut self, path: &str) -> std::io::Result<()>{
		let dir_iter = fs::read_dir(path)?;
		let mut count_dir = 0;
		for content_res in dir_iter{
			let content = content_res?;
			if content.file_type()?.is_dir(){
				count_dir +=1;
				let path_dir = content.path().to_str().expect("FAILED").to_string();
				self.read_directory(&path_dir);
			}else if content.file_type()?.is_file(){
				let path_file = content.path().to_str().expect("FAILED").to_string();
				match self.read_id3(&path_file){
					Ok(_) =>println!("{}", path_file),
					Err(e) => print!(""),
				};
			}
		}
		Ok(())
	}

	pub fn title_song_query(&self, condition_value: &String)-> Option<Vec<Song>>{
		let song_controller = SongController::new(self.db_connection.get_connection());
		song_controller.get_query_with(condition_value)
	}
	pub fn get_last_songs(&mut self) -> &mut Vec<Song>{
		&mut self.last_songs
	}
	pub fn get_all_songs(&self) -> Option<Vec<Song>>{
		let song_controller = SongController::new(self.db_connection.get_connection());
		song_controller.get_all_songs()
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	use rusqlite::{params, Connection, Result};
	
	#[test]
	fn test_extract_id3() -> Result<()>{
		let db = DBConnection::establish("src/models/db/music.db")?;
		let mut miner = Miner::new(db.clone());
		let path = "/home/marco/Desktop/modelado/mi_album";
		//miner.read_directory(path);
		Ok(())
	}
}
