use rusqlite::{params, Connection, Result};
use crate::models::performer::Performer;
use crate::models::types::Types;
pub struct PerformerDao{
	connection: Connection,
}
impl PerformerDao{
	pub fn new() -> Result<Self>{
		let connection = Connection::open("src/models/db/music.db").expect("failed open db");
		Ok(PerformerDao{
			connection,
		})
	}
	pub fn create_perform_table(&self) -> Result<()>{
		self.connection.execute(
			"CREATE TABLE IF NOT EXISTS performers (
				id_performer INTEGER PRIMARY KEY AUTOINCREMENT,
				id_type INTEGER,
				name TEXT,
				FOREIGN KEY (id_type) REFERENCES types(id_type)
			)",(),)?;
		Ok(())
	}
	pub fn add_perform(&self, performer: &Performer) -> Result<()>{
		self.connection.execute(
			"INSERT INTO performers (id_type, name) VALUES (?1)",(performer.get_type().get_id_type(), performer.get_name()),
		)?;
		Ok(())
	}
	pub fn get_performers(&self) -> Result<Vec<Performer>>{
		let mut stmut = self.connection.prepare(
			"SELECT name, description FROM performers JOIN types ON 
			peformers.id_types = types.id_types
			")?;
		let perform_rows = stmut.query_map([], |row| {
			let types = Types::new(row.get(1)?);
			let performer = Performer::new(row.get(0)?, types);
			Ok(performer)
		})?;
		let mut perform_iter = Vec::new();
		for performer in perform_rows{
			if let Ok(performer_res) = performer{
				perform_iter.push(performer_res);
			} 
		}
		Ok(perform_iter)
	}
	
}
