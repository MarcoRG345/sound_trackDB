use std::sync::{Arc, Mutex};
use rusqlite::{params, Connection, Result};

#[derive(Clone)]
pub struct DBConnection{
	connection: Arc<Mutex<Connection>>
}

impl DBConnection{
	pub fn establish(path: &str) -> Result<Self>{
		let connection = Arc::new(Mutex::new(Connection::open(path)?));
		Ok(DBConnection { connection, })
	}
	pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
		Arc::clone(&self.connection)
	}
}
