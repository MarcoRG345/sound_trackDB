## Release v0.1.0 - Pre-Alpha
# Description
This is the initial release of Sound TrackDB, a musical database implementation. This version is a pre-alpha and has limited functionalities.
Can manage your own mp3 files and save them a binary data base. Can search songs for album name, performer name, title song, it will show you
the results. The project was implemented 100% in rust and use gtk, gio tecnologies to use them in the GUI interface context, and use sqlite for
the logic data base.

# Use
Can use this software with two main ways:
 * Compile using cargo with `cargo build` first, then use `cargo run` it will be slow compiling and perhaps the computer will work hard,
   but this it only the first time compiling. I DONT`t recommend use 'cargo clean' so easy, i has not found a way to reduce de compiling resources time yet.
 * Download the sound_tracDB in the "releases" section, descompress it and execute `./sound_trackDB` binary.

# Features
   Ability to store MP3 files in the database.
    Read ID3 tags from MP3 files and display the information.
    Getting queries and show them
    A basic GUI presentation.

# Limitations
   The application currently shows duplicate entries of files in the database.
    The functionality to play MP3 files is not included in this version.
    Getting queries but you can not edit songs and albums yet
    The GUI is so basic it will designed more cyberpunk style or something modern context.
    
# Notes
This version is intended for testing and feedback. Additional features and bug fixes will be addressed in future releases.
