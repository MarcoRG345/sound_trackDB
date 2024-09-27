#[derive(Debug)]
pub struct Types{
	id_type: i32,
	description: String,
}
impl Types{
	pub fn new(description: String) -> Self{
		Types{
			id_type: 0,
			description,
		}
	}
	pub fn get_id_type(&self) -> &i32{
		&self.id_type
	}
	pub fn get_description(&self) -> &String{
		&self.description
	}
	pub fn set_id_type(&mut self, id_type: i32){
		self.id_type = id_type;
	}
}
