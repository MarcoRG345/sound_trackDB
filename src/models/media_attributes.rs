#[derive(Debug)]
pub struct MediaAttributes {
	path: String,
	year: u32,
}
impl MediaAttributes{
	pub fn new(path: String, year: u32) -> Self{
		MediaAttributes { path,  year, } 
	}
	pub fn get_path(&self) -> &String {
		&self.path
	}
	pub fn get_year(&self) -> &u32{
		&self.year
	}
	pub fn set_path(&mut self, path: &str){
		self.path = path.to_string();
	}
}
