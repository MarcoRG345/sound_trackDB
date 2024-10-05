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
	metadata: HashMap<String, String>,
}

impl Miner{
	pub fn new() -> Self{
		let mut metadata = HashMap::new();
		let db_connection = DBConnection::establish("src/models/db/music.db").expect("failed");
		Miner{
			db_connection,
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

	pub fn insert_database(&self, path: &str){
		let song_controller = SongController::new(self.db_connection.get_connection());
		let album_controller = AlbumController::new(self.db_connection.get_connection());
		let performer_controller = PerformerController::new(self.db_connection.get_connection());
		
		let type_controller = TypeController::new(self.db_connection.get_connection());
		//show dialog to ask for user.
		let types = Types::new("Person".to_string());
		type_controller.create_types();
		
		let mut album = Albums::new(path.to_string(),
								self.metadata.get("album").unwrap_or(&"Unknown".to_string()).to_string(),
								self.metadata.get("year").unwrap().parse().unwrap());
		let id_album = album_controller.add_album(&album);
		album.set_id_album(id_album);
		
		let mut performer = Performer::new(self.metadata.get("artist").unwrap_or(&"Unknown".to_string()).to_string(), types);	
		let id_perform = performer_controller.add_perform(&performer);
		performer.set_id(id_perform);
		
		let access =  MediaAttributes::new(path.to_string(), self.metadata.get("year").unwrap().parse().unwrap());
		let song = Song::new(performer, album, self.metadata.get("title").unwrap_or(&"Unknown".to_string()).to_string(),
							 access, 3, self.metadata.get("genre").unwrap_or(&"Unknown".to_string()).to_string());
		song_controller.add_song(&song);
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
	
}

#[cfg(test)]
mod tests{
	use super::*;
	
	#[test]
	fn test_extract_id3(){
		let mut miner = Miner::new();
		let path = "/home/marco/Desktop/modelado/mi_album";
		miner.read_directory(path);
	}
}
