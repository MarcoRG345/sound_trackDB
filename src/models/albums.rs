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
}
