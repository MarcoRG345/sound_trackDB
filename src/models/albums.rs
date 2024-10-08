#[derive(Debug)]
pub struct Albums{
	id_album: i32,
	path: String,
	name: String,
	year: u32,
}
impl Albums {
	pub fn new(id_album: i32, path: String, name: String, year:u32) -> Self{
		Albums{
			id_album,
			path,
			name,
			year,
		}
	}
	pub fn empty_new() -> Self{
		Albums{
			id_album: 0,
			path: "init".to_string(),
			name: "init_name".to_string(),
			year: 2000,
		}
	}
	pub fn get_id(&self) -> &i32{
		&self.id_album
	}
	pub fn get_path(&self) -> &String{
		&self.path
	}
	pub fn get_name(&self) -> &String{
		&self.name
	}
	pub fn get_year(&self) -> &u32{
		&self.year
	}
	pub fn set_path(&mut self, path: String){
		self.path = path;
	}
	pub fn set_id_album(&mut self, id_album: i32){
		self.id_album = id_album;
	}
	pub fn set_name(&mut self, name: String){
		self.name = name;
	}
	pub fn set_year(&mut self, year: u32){
		self.year = year;
	}
}
