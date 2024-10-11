mod models;
mod controllers;
mod views;
use crate::views::interface::GUInterface;
use crate::models::db::album_dao::Album_dao;
use crate::models::albums::Albums;
use crate::models::performer::Performer;
use crate::models::db::performer_dao::PerformerDao;
use std::sync::{Arc, Mutex};
use std::fs::File;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{FileChooserButton, Button, ListBox, ListBoxRow, Label, Builder, Window, Application, ApplicationWindow};
use std::env;
use crate::models::db::db_connection::DBConnection;
use crate::controllers::miner::Miner;


fn main(){
	let app = Application::new(Some("com.example.GtkMusicApp"), Default::default()).expect("failed");
	let db_connection = DBConnection::establish("src/models/db/music.db").expect("failed");
	let miner = Arc::new(Mutex::new(Miner::new(db_connection.clone())));

	app.connect_activate(move |app| {
		let glade_path = include_str!("views/gui_scheme.glade");
		let builder = Builder::from_string(glade_path);
		
		let window: Window = builder.get_object("mainWindow").expect("failed");
		window.set_application(Some(app));
		let apply: Button = builder.get_object("Apply").expect("failed");
		let mut view = GUInterface::new(&builder, miner.clone());
		view.connect_signals();
		view.apply_changes();
		window.show_all();
	});
	app.run(&[]);

}
