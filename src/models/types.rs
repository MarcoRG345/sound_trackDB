#[derive(Debug)]
pub struct Types{
	id_type: u32,
	description: String,
}
impl Types{
	pub fn new(id_type: u32, description: String) -> Self{
		Types{
			id_type,
			description,
		}
	}
	pub fn get_id_type(&self) -> &u32{
		&self.id_type
	}
	pub fn get_description(&self) -> &String{
		&self.description
	}
	
}
