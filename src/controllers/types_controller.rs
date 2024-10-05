use crate::models::db::types_dao::TypesDao;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use crate::models::types::Types;


pub struct TypeController{
	typeDao: TypesDao,
}

impl TypeController{
	pub fn new (connection: Arc<Mutex<Connection>>) -> Self{
		let typeDao = TypesDao::new(connection);
		TypeController{
			typeDao,
		}
	}
	pub fn create_types(&self){
		self.typeDao.create_types_table();
	}
}
