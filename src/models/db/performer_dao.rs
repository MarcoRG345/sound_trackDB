use rusqlite::{params, Connection, Result};
use crate::models::performer::Performer;

pub struct Performer_dao{
	connection: Connection,
}
impl Performer_dao{
	pub fn new() -> Result<Self>{
		let connection = Connection::open("src/models/db/music.db").expect("failed open db");
		Ok(Performer_dao{
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
}
