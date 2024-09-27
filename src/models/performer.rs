use crate::models::types::Types;

#[derive(Debug)]
pub struct Performer{
	id_performer: i32,
	name: String,
	types: Types,
	
}

impl Performer{
	pub fn new(name: String, types: Types) -> Self{
		Performer{
			id_performer: 0,
			name,
			types,
		}
	}
	pub fn get_id_perf(&self) -> &i32{
		&self.id_performer
	}
	pub fn get_name(&self) -> &String{
		&self.name
	}
	pub fn get_type(&self) -> &Types{
		&self.types
	}
}
