// use fs to access
// the filesystem and read from a directory
use std::{
    fs::read_dir,
    env::set_current_dir,
    iter::Iterator
};

// import the needed trait
use crate::widgets::traits::CustomList; 

// FileList struct
// Gets used by draw_layout
// to draw the list widget which displays files
pub struct FileList {
    
    pub current: usize, // current selected item
    pub content: Vec<String>, // all items
    pub update: bool // if update is needed

}

impl FileList {

    // get the element of the current directory
    // and update self.content
    fn get_dir() -> Vec<String> {
        
        read_dir("./")
            .expect("Could not read directory!")
            .map(|res| res.map(|e| e.path().to_str().unwrap().to_string())) // get the path and put it in a list
            .map(|x| {
                let mut x = x.unwrap();
                if x.len() > 2 {
                    x.remove(0); x.remove(0); // remove the ./ prefix

                }; x
            }).collect::<Vec<String>>()

    }

    // creates a new file list with
    // the content of the current directory
    pub fn new() -> Self {

        // get all elements off the cwd
        let cwd_content = Self::get_dir();

        // return the FileList struct
        Self {

            current: 0,
            content: cwd_content,
            update: true

        }
    
    }

    // gets the current selected element
    pub fn get_current(&self) -> String {
        self.content[self.current].clone()
    }

    // update the list
    pub fn update(&mut self) {
        if self.update {
            // get the files
            let files = Self::get_dir();
            if files.is_empty() {
                self.content = vec!["Nothing found!".to_string()];
            } else {
                self.content = files;
            }
        }
    }

    // change one directory back
    pub fn change_dir_back(&mut self) {
        // get all elements off the cwd
        set_current_dir("..").expect("Not possible to change back!");
        self.update = true;
    }

    // change directory to current selected element
    pub fn change_dir_selected(&mut self) {

        // current selected element
        let path = &self.content[self.current];
        match set_current_dir(path.as_str()) {
            Ok(_) => {},
            Err(_) => {}
        }; self.update = true;

    }

    // sort the files after the input string
    pub fn sort(&mut self, key: String) {

        if key.len() == 0 { 
            self.content = Self::get_dir();
            return;
        }
            
        // empty the whole list
        self.content = Vec::new();
        // get all files of the cwd
        let current_filelist = Self::get_dir();

        // create new key
        for n in &current_filelist {
            if n.contains(&key) { 
                self.content.push(n.clone());
            }
        }
         
        if self.content.is_empty() {
             self.content = vec!["Nothing found!".to_string()];
        }

        self.update = false;

    }

}

impl CustomList for FileList {

    fn get_len(&self) -> usize {
        self.content.len()
    }

    fn get_current(&self) -> usize {
        self.current
    }

    fn set_current(&mut self, new: usize) {
        self.current = new;
    }

}
