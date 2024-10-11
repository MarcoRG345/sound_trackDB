use gtk::prelude::*;
use gio::File;
use gio::prelude::*;
use gtk::{FileChooserButton, Box,Button, ListBox, ListBoxRow, Label, Builder, Window, Application, ApplicationWindow};
use std::env;
use crate::controllers::miner::Miner;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct GUInterface{
	miner: Arc<Mutex<Miner>>,
	listBox: Arc<Mutex<ListBox>>,
	file_chooser:FileChooserButton,
	apply: Button,
}
impl GUInterface{
	
	pub fn new(builder: &Builder, miner: Arc<Mutex<Miner>>) -> Self{
		let apply: Button = builder.get_object("Apply").expect("failed");
		let mut listBox: Arc<Mutex<ListBox>> = Arc::new(Mutex::new(builder.get_object("ListBox").expect("failed")));
		let mut file_chooser: FileChooserButton = builder.get_object("FileChooserButton").expect("failed");
		Self {
	  		miner,
			listBox,
			file_chooser,
			apply,
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

		
    	    println!("ID: {}", song.get_id());  // Imprime los valores para asegurarte que no están vacíos
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
		mine_key.get_last_songs().clear();
	}

}

