// import the needed trait
use crate::widgets::traits::{
    CustomParagraph,
    EditableParagraph
};

use tui::style::{ Style, Color };

pub struct Search {

    pub content: String,
    pub border_style: Style // border colors

}

impl Search {

    // create a new widget 
    pub fn new(bs: [u8; 3]) -> Search {
        Search {
            content: String::new(),
            border_style: Style::default().fg(Color::Rgb(bs[0], bs[1], bs[2]))
        }
    }

    // updates the string
    // with the input char
    pub fn add(&mut self, new: String) {
        self.content = format!("{}{}", self.content, new)
    }

    // pop the last element of the string
    // = Backspace
    pub fn delete(&mut self) {
        self.content.pop();
    }

    // clear the content
    // get called when new search started
    pub fn clear(&mut self) {
        self.content = String::new()
    }

}

impl CustomParagraph for Search  {

    fn items(&self) -> String {
        self.content.clone()
    }

}

impl EditableParagraph for Search {

    fn get_content(&self) -> String {
        self.content.clone()
    }

    fn set_content(&mut self, new: String) {
        self.content = new;
    }

}
