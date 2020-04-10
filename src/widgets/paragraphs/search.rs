// import the needed trait
use crate::widgets::traits::{
    CustomParagraph,
    EditableParagraph
};

pub struct Search {

    pub content: String

}

impl Search {

    // create a new widget 
    pub fn new() -> Search {
        Search {
            content: String::new()
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
