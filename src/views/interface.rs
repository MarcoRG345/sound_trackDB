use gtk::prelude::*;
use gio::File;
use gio::prelude::*;
use gtk::{FileChooserButton, Entry, Box, Button, ListBox, ListBoxRow, Label, Builder, Window, Application, ApplicationWindow};
use std::env;
use crate::controllers::miner::Miner;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
pub struct GUInterface{
	miner: Arc<Mutex<Miner>>,
	listBox: Arc<Mutex<ListBox>>,
	file_chooser:FileChooserButton,
	apply: Button,
	find_button: Button,
	entry: Arc<Mutex<Entry>>,
	filtered_box: Arc<Mutex<ListBox>>,
}
impl GUInterface{
	
	pub fn new(builder: &Builder, miner: Arc<Mutex<Miner>>) -> Self{
		let apply: Button = builder.get_object("Apply").expect("failed");
		let find_button: Button = builder.get_object("FindButton").expect("failed");
		let entry: Arc<Mutex<Entry>> = Arc::new(Mutex::new(builder.get_object("entryButton").expect("failed")));
		let mut listBox: Arc<Mutex<ListBox>> = Arc::new(Mutex::new(builder.get_object("ListBox").expect("failed")));
		let mut filtered_box: Arc<Mutex<ListBox>> = Arc::new(Mutex::new(builder.get_object("filteredBox").expect("failed")));
		let mut file_chooser: FileChooserButton = builder.get_object("FileChooserButton").expect("failed");
		Self {
	  		miner,
			listBox,
			file_chooser,
			apply,
			find_button,
			entry,
			filtered_box,
		}
	}

	pub fn connect_signals(&mut self){
		let miner_clone = Arc::clone(&mut self.miner);
		let listBox_clone = Arc::clone(&mut self.listBox);
		self.file_chooser.connect_file_set(move |file_widget| {
			if let Some(file_path) = file_widget.get_filename(){
				let mut miner_key = miner_clone.lock().unwrap();
				let mut list_box_key = listBox_clone.lock().unwrap();			
				miner_key.read_directory(file_path.to_str().expect("failed"));
			}
		});
	}
	pub fn apply_changes(&mut self){
		let miner_clone = Arc::clone(&mut self.miner);
		let listBox_clone = Arc::clone(&mut self.listBox);
		self.apply.connect_clicked(move |_| {
			Self::update_listbox(miner_clone.clone(), listBox_clone.clone());
		});
	}
	
	pub fn update_listbox(miner:Arc<Mutex<Miner>>, lbox:Arc<Mutex<ListBox>>){
		let mut mine_key = miner.lock().unwrap();
		
		
		let lbox_key = lbox.lock().unwrap();
		for song in mine_key.get_last_songs().iter(){
			let register_button = Button::new();
			let box_container = Box::new(gtk::Orientation::Horizontal, 5);
			let id_label = Label::new(Some(&song.get_id().to_string()));
			let title_label = Label::new(Some(song.get_tittle()));
			let performer_label = Label::new(Some(song.get_performer().get_name()));
			let album_label = Label::new(Some(song.get_album().get_name()));
			let year_label = Label::new(Some(&song.get_album().get_year().to_string()));
			let genre_label = Label::new(Some(song.get_genre()));

			

			box_container.pack_start(&id_label, true, true, 0);
			box_container.pack_start(&title_label, true, true, 0);
			box_container.pack_start(&performer_label, true, true, 0);
			box_container.pack_start(&album_label, true, true, 0);
			box_container.pack_start(&year_label, true, true, 0);
			box_container.pack_start(&genre_label, true, true, 0);
		
			register_button.add(&box_container);
			lbox_key.add(&register_button);
			
		}
		lbox_key.show_all();
		mine_key.get_last_songs().clear();
	}

	pub fn connect_entry(&self){
		let entry_clone = Arc::clone(&self.entry);
		let miner_clone = Arc::clone(&self.miner);
		let filtered_box_clone = Arc::clone(&self.filtered_box);
		self.find_button.connect_clicked(move |_| {
			
			let mut data_entry = entry_clone.lock().unwrap().get_text().to_string().trim().to_string();
			Self::update_query_box(&data_entry.to_string(), miner_clone.clone(), filtered_box_clone.clone());
		});
	}

	pub fn update_query_box(data_entry: &String, miner:Arc<Mutex<Miner>>,lbox: Arc<Mutex<ListBox>>){
		let mut miner_key = miner.lock().unwrap();
		let lbox_key = lbox.lock().unwrap();
		let songs_iter_option = miner_key.title_song_query(data_entry);
		lbox_key.get_children().into_iter().for_each(|child| {
			lbox_key.remove(&child);
		});
		match songs_iter_option{
			Some(songs_it) => {
				println!("Si hay algo que enseñar");

				for song in songs_it.iter(){
					let register_button = Button::new();
					let box_container = Box::new(gtk::Orientation::Horizontal, 5);
					let id_label = Label::new(Some(&song.get_id().to_string()));
					let title_label = Label::new(Some(song.get_tittle()));
					let performer_label = Label::new(Some(song.get_performer().get_name()));
					let album_label = Label::new(Some(song.get_album().get_name()));
					let year_label = Label::new(Some(&song.get_album().get_year().to_string()));
					let genre_label = Label::new(Some(song.get_genre()));

		
    	   			println!("ID: {}", song.get_id());
    	    		println!("Título: {}", song.get_tittle());
    	    		println!("Intérprete: {}", song.get_performer().get_name());
   					println!("Álbum: {}", song.get_album().get_name());

					box_container.pack_start(&id_label, true, true, 0);
					box_container.pack_start(&title_label, true, true, 0);
					box_container.pack_start(&performer_label, true, true, 0);
					box_container.pack_start(&album_label, true, true, 0);
					box_container.pack_start(&year_label, true, true, 0);
					box_container.pack_start(&genre_label, true, true, 0);

					register_button.add(&box_container);
					lbox_key.add(&register_button);
				}
				lbox_key.show_all();
				
			},
			None => {
				let error_label = Label::new(Some("Query error, no found song"));
				lbox_key.add(&error_label);
				lbox_key.show_all();
				println!("NO hay nada que enseñar");
			},
		}
	}
}

