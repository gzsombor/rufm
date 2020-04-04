use std::{
    fs::File,
    fs::read_dir,
    path::Path,
    io::prelude::*
};

use tui::widgets::{
    Text
};

extern crate alloc;
use alloc::borrow::Cow;

// import the needed trait
use crate::widgets::traits::CustomParagraph; 

pub struct Preview {

    pub filename: String, // the name of the previewed file
    pub content: String // the content of the previewed file

}

impl Preview {

    // get the element of the current directory
    // and update self.content
    fn get_dir(name: String) -> Vec<String> {
        
        read_dir("./".to_string() + &name)
            .expect("Could not read directory!")
            .map(|res| res.map(|e| e.path().to_str().unwrap().to_string())) // get the path and put it in a list
            .map(|x| {
                let mut x = x.unwrap();
                if x.len() > 2 {
                    x.remove(0); x.remove(0); // remove the ./ prefix

                }; x
            }).collect::<Vec<_>>() // .collect::<Result<Vec<_>, Error>>().unwrap()

    }

    // create a new empty struct
    pub fn new() -> Preview {
        Preview {
            filename: String::new(),
            content: String::new()
        }
    }

    pub fn set_filename(&mut self, new: String) {
        // update the filename
        self.filename = new;
    }

    pub fn update_content(&mut self) {
        // clear the string
        self.content = String::new();
        // check if the filename points to
        // a directory of a file
        if Path::new(&self.filename).is_dir() {
            let dir_content = Preview::get_dir(self.filename.clone());
            for i in dir_content {
                self.content.push_str(&(i + "\n"));
            }

            return;
        } 
        
        // open file
        let mut file = File::open
            (self.filename.clone());
       
        // check if file could open
        match file {
            Ok(mut f) => {
                // parse content into content variable
                f.read_to_string(&mut self.content);
            },
            Err(e) => {
                self.content = "No preview avaible!".to_string();
            }
        }
    }

}

impl CustomParagraph for Preview {

    fn items(&self) -> String {
        self.content.clone()
    }

}
