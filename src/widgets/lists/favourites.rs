// use fs to access
// the filesystem and read from a directory
use std::{
    env::set_current_dir
};

use tui::style::{ Style, Color };

// import the needed trait
use crate::widgets::traits::CustomList; 

// FileList struct
// Gets used by draw_layout
// to draw the list widget which displays files
pub struct Favourites {

    pub current: usize, // current selected item
    pub names: Vec<String>, // all names
    pub paths: Vec<String>, // all paths (same index as names)
    pub border_style: Style // the border colors

}

impl Favourites {

    // creates a new file list with
    // the content of the current directory
    pub fn new(bs: [u8; 3], input_names: Vec<String>, input_paths: Vec<String>) -> Favourites {

        // return the FileList struct
        Favourites {

            current: 0,
            names: input_names,
            paths: input_paths,
            border_style: Style::default().fg(Color::Rgb(bs[0], bs[1], bs[2]))

        }
    
    }

    // change directory to current selected element
    pub fn change_dir_selected(&mut self) {

        // current selected element
        let path = &self.paths[self.current];
        set_current_dir(path.as_str()).expect("Could not change the directory!");

    }

}

impl CustomList for Favourites {
 
    fn get_len(&self) -> usize {
        self.names.len()
    }

    fn get_current(&self) -> usize {
        self.current
    }

    fn set_current(&mut self, new: usize) {
        self.current = new;
    }

}
