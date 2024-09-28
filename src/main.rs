mod models;
mod controllers;
use crate::models::db::album_dao::Album_dao;
use crate::models::albums::Albums;
use crate::models::performer::Performer;
use crate::models::db::performer_dao::PerformerDao;
use std::sync::{Arc, Mutex};

fn main(){
	println!("Entry point");
}
