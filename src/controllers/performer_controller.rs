use crate::models::db::performer_dao::PerformerDao;
use crate::models::performer::Performer;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};

pub struct PerformerController{
	performerDao: PerformerDao,
	
}
impl PerformerController{
	pub fn new(connection: Arc<Mutex<Connection>>) -> Self{
		let performerDao = PerformerDao::new(connection);
		PerformerController{
			performerDao,
		}
	}
	pub fn add_perform(&self, performer: &Performer) -> i64{
		self.performerDao.create_perform_table();
		let id: i64 = match self.performerDao.add_perform(performer){
			Ok(id_) => id_,
			_ => 0,
		};
		id
	}
}
