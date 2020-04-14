// import the needed trait
use crate::widgets::traits::{
    CustomParagraph,
    EditableParagraph
};

use std::{
    fs::metadata
};

pub enum InfoMode {
    Status,
    Information,
    Confirmation,
    Input
}

pub struct Info {

    pub content: String,
    pub mode: InfoMode // the current mode

}

impl Info {

    // create a new info widget
    pub fn new() -> Self {
        Self {
            content: String::new(),
            mode: InfoMode::Information
        }
    }

    // update the content with information on file
    pub fn update(&mut self, name: String) {
    
        match self.mode {

            InfoMode::Information => match metadata(name) {

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

            },

            InfoMode::Status => self.mode = InfoMode::Information,

            _ => {}

        }

    }

}

impl CustomParagraph for Info {

    fn items(&self) -> String {
        self.content.clone()
    }

}

impl EditableParagraph for Info {

    fn get_content(&self) -> String {
        self.content.clone()
    }

    fn set_content(&mut self, new: String) {
        self.content = new;
    }

}
