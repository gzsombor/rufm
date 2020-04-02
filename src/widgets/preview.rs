use std::{
    fs::File,
    io::prelude::*
};

use tui::widgets::{
    Text
};

pub struct Preview {

    pub filename: String, // the name of the previewed file
    pub content: String // the content of the previewed file

}

impl Preview {

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
        // open file
        let mut file = File::open
            (self.filename.clone());
       
        // check if file could open
        match file {
            Ok(mut f) => {
                // parse content into content variable
                self.content = String::new();
                f.read_to_string(&mut self.content);
            },
            Err(e) => {
                self.content = "No preview avaible!".to_string();
            }
        }
    }

    pub fn get_content(&mut self) -> Vec<Text> {
        // make every string to an Text::raw element
        vec![Text::raw(self.content.clone())]
    }

}
