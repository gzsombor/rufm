// import the needed trait
use crate::widgets::traits::CustomParagraph; 

use std::{
    fs::metadata
};

pub struct Info {

    pub content: String

}

impl Info {

    // create a new info widget
    pub fn new() -> Self {
        Self {
            content: String::new()
        }
    }

    // update the content with information on file
    pub fn update(&mut self, name: String) {

        // create metadata
        let md = metadata(name).unwrap();
        // get the info
        let len = md.len();
        let kind = if md.is_dir() { "directory" } else { "file     " };
        let readonly = if md.permissions().readonly() { "read only   " } else { "read & write" };
        // update the content var
        self.content = format!("{}  {}  {:>5}", kind, readonly, len);

    }

}

impl CustomParagraph for Info {

    fn items(&self) -> String {
        self.content.clone()
    }

}
