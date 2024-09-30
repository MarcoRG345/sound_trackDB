use id3::{Tag, TagLike, Content};
use crate::models::db::db_connection::DBConnection;
use crate::models::db::performer_dao::PerformerDao;
use crate::models::db::types_dao::TypesDao;
use crate::models::db::album_dao::Album_dao;
use crate::models::performer::Performer;
use crate::models::media_attributes::MediaAttributes;
use crate::models::song::Song;
use crate::models::types::Types;
use crate::models::albums::Albums;
use std::fs;

pub fn read_id3(path: &str) -> Result<(), Box<dyn std::error::Error>>{
	let file_tags = Tag::read_from_path(path)?;
	let mut tpe1_performer = String::new();
	let mut tit2_title = String::new();
	let mut talb_album = String::new();
	let mut tcon_genre = String::new();
	let mut trck_track = String::new();
	for frame in file_tags.frames(){
		match frame.id(){
			"TPE1" => {
				tpe1_performer = frame.content().to_string();
			},
			"TIT2" => {
				tit2_title = frame.content().to_string();
			},
			"TALB" => {
				talb_album = frame.content().to_string();
			},
			"TDRC" => {
				
			},
			"TDRC" => {
				tcon_genre = frame.content().to_string();
			},
			"TRCK" => {
				trck_track = frame.content().to_string();
			},
			_ => {print!("");},
		}
	}
	println!("artista: {}", tpe1_performer);
	println!("titulo: {}", tit2_title);
	println!("genero: {}", talb_album);
	println!("pista: {}", trck_track);
	Ok(())
	
}

pub fn read_directory(path: &str) -> std::io::Result<()>{
	let dir_iter = fs::read_dir(path)?;
	let mut count_dir = 0;
	for content_res in dir_iter{
		let content = content_res?;
		if content.file_type()?.is_dir(){
			count_dir +=1;
			let path_dir = content.path().to_str().expect("FAILED").to_string();
			read_directory(&path_dir);
		}else if content.file_type()?.is_file(){
			let path_file = content.path().to_str().expect("FAILED").to_string();
			match read_id3(&path_file){
				Ok(_) =>println!("{}", path_file),
				Err(e) => print!(""),
			};
		}
	}
	Ok(())
}  

#[cfg(test)]
mod tests{
	use super::*;
	
	#[test]
	fn test_extract_id3(){
		let path = "/home/marco/Desktop/modelado/mi_album/";	
		read_directory(path);
	}
}
