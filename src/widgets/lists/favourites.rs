// use fs to access
// the filesystem and read from a directory
use std::{
    env::set_current_dir,
    iter::Iterator
};

// import the needed trait
use crate::widgets::traits::CustomList; 

// style and color selected row,
// display text
use tui::style::{Color, Style};

// FileList struct
// Gets used by draw_layout
// to draw the list widget which displays files
pub struct Favourites {

    pub current: usize, // current selected item
    pub names: Vec<String>, // all names
    pub paths: Vec<String> // all paths (same index as names)

}

impl Favourites {

    // creates a new file list with
    // the content of the current directory
    pub fn new(names: Vec<String>, paths: Vec<String>) -> Favourites {

        // return the FileList struct
        Favourites {

            current: 0,
            names: names,
            paths: paths 
        }
    
    }

    // change directory to current selected element
    pub fn change_dir_selected(&mut self) {

        // current selected element
        let path = &self.paths[self.current];
        set_current_dir(path.as_str()); // path.as_str());

    }

//    pub fn select(&mut self) -> Vec<Text<'_>> {
//
//        self.content.iter().enumerate().map(|(index, f)| {
//
//            if index == self.current {
//
//                Text::Styled(
//                    Cow::Borrowed(f),
//                    self.highlight
//                )
//
//            } else {
//
//                Text::Raw(Cow::Borrowed(f))
//
//            }
//
//        }).collect()
//
//    }

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
