// use fs to access
// the filesystem and read from a directory
use std::{
    env::set_current_dir
};

// import the needed trait
use crate::widgets::traits::CustomList; 

// FileList struct
// Gets used by draw_layout
// to draw the list widget which displays files
pub struct Favourites {

    pub current: usize, // current selected item
    pub names: Vec<String>, // all names
    pub paths: Vec<String>, // all paths (same index as names)
    pub update: bool // if update is needed

}

impl Favourites {

    // creates a new file list with
    // the content of the current directory
    pub fn new(names: Vec<String>, paths: Vec<String>) -> Favourites {

        // return the FileList struct
        Favourites {

            current: 0,
            names: names,
            paths: paths,
            update: true

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
