use std::{
    fs::File,
    fs::read_dir,
    path::Path,
    io::prelude::*
};

// import the needed trait
use crate::widgets::traits::CustomParagraph; 

pub struct Preview {

    pub filename: String, // the name of the previewed file
    pub content: String, // the content of the previewed file
    pub update: bool // if it should update

}

impl Preview {

    // get the element of the current directory
    // and update self.content
    fn get_dir(&mut self) {
        
        match read_dir("./".to_string() + &self.filename) {
            Ok(v) => {
                let dir_content = v.map(|res| res.map(|e| e.path().to_str().unwrap().to_string())) // get the path of all elements
                    .map(|x| {
                        let mut x = x.unwrap();
                        if x.len() > 2 {
                            x.remove(0); x.remove(0); // remove the ./ prefix
                        }; x
                    }).collect::<Vec<_>>();
                for i in dir_content {
                    self.content.push_str(&(i + "\n"));
                } 
            },
            Err(_) => self.content = "No preview avaible!".to_string()
        }

    }

    // create a new empty struct
    pub fn new() -> Self {
        Self {
            filename: String::new(),
            content: String::new(),
            update: true
        }
    }

    pub fn update(&mut self, new: String) {

        if self.update {
            // update the filename
            self.filename = new;
            // clear the string
            self.content = String::new();
            // check if the filename points to
            // a directory of a file
            let path = Path::new(&self.filename);
            
            // check if path exists
            if !path.exists() {
                self.content.push_str("No preview avaible!");
                return;
            }

            if path.is_dir() {
                self.get_dir();
                return;
            } 
            
            // open file
            let file = File::open
                (self.filename.clone());
       
            // check if file could open
            match file {
                Ok(mut f) => {
                    // parse content into content variable
                    match f.read_to_string(&mut self.content) {
                        Ok(_) => {},
                        Err(_) => self.content = "No preview avaible!".to_string()
                    }
                },
                Err(_) => {
                    self.content = "No preview avaible!".to_string();
                }
            }
        }
        
    }

}

impl CustomParagraph for Preview {

    fn items(&self) -> String {
        self.content.clone()
    }

}
