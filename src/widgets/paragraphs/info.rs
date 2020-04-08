// import the needed trait
use crate::widgets::traits::CustomParagraph; 

use std::{
    fs::metadata
};

pub struct Info {

    pub content: String,
    pub update: bool

}

impl Info {

    // create a new info widget
    pub fn new() -> Self {
        Self {
            content: String::new(),
            update: true
        }
    }

    // update the content with information on file
    pub fn update(&mut self, name: String) {
    
        if self.update {
            // create metadata
            match metadata(name) {
                Ok(v) => {
                    let md = v;
                    // get the info
                    let len = md.len();
                    let kind = if md.is_dir() { "d" } else { "f" };
                    let readonly = if md.permissions().readonly() { "r " } else { "rw" };
                    // update the content var
                    self.content = format!("{}  {}  {:>5}", kind, readonly, len);
                },
                Err(_) => {
                    self.content = "No information avaible!".to_string();
                }
            };
        } else {
            self.update = true;  
        }
    }

}

impl CustomParagraph for Info {

    fn items(&self) -> String {
        self.content.clone()
    }

}
