use crate::models::performer::Performer;
use crate::models::albums::Albums;
use crate::models::media_attributes::MediaAttributes;

#[derive(Debug, Clone)]
pub struct Song{
	id_song: i64,
	performer: Performer,
	album: Albums,
	tittle: String,
	access: MediaAttributes,
	track: i32,
	genre: String,
}

impl Song{
	pub fn new(performer: Performer, album: Albums, tittle: String, access: MediaAttributes, track: i32, genre:String) -> Self {
		Song{
			id_song: 0,
			performer,
			album,
			tittle,
			access,
			track,
			genre,
		}
	}
	pub fn get_performer(&self)-> &Performer{
		&self.performer
	}
	pub fn get_album(&self) -> &Albums{
		&self.album
	}
	pub fn get_tittle(&self) -> &String{
		&self.tittle
	}
	pub fn get_access(&self) -> &MediaAttributes{
		&self.access
	}
	pub fn get_track(&self) -> &i32 {
		&self.track
	}
	pub fn get_genre(&self) -> &String{
		&self.genre
	}
	pub fn set_id(&mut self, id_song: i64){
		self.id_song = id_song;
	}
	pub fn get_id(&self) -> &i64{
		&self.id_song
	}
}
