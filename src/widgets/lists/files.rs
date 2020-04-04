// use fs to access
// the filesystem and read from a directory
use std::{
    fs::read_dir,
    env::set_current_dir,
    iter::Iterator
};

// import the needed trait
use crate::widgets::traits::ScrollableList; 

// style and color selected row,
// display text
use tui::style::{Color, Style};

// FileList struct
// Gets used by draw_layout
// to draw the list widget which displays files
pub struct FileList {

    pub current: usize, // current selected item
    pub content: Vec<String>, // all items
    pub highlight: Style // the style, which the current item should have

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
            }).collect::<Vec<_>>()

    }

    // creates a new file list with
    // the content of the current directory
    pub fn new() -> FileList {

        // get all elements off the cwd
        let cwd_content = FileList::get_dir();

        // create the hightlighting style
        let style = Style::default().fg(Color::White).bg(Color::Blue);
        
        // return the FileList struct
        FileList {

            current: 0,
            content: cwd_content,
            highlight: style

        }
    
    }

    // update the list
    pub fn update(&mut self) {
        if FileList::get_dir().is_empty() {
            self.content = vec!["Nothing found!".to_string()];
        } else {
            self.content = FileList::get_dir();
        }
    }

    // change one directory back
    pub fn change_dir_back(&mut self) {

        // get all elements off the cwd
        set_current_dir("..");
        
        // update the content
        self.update();

    }

    // change directory to current selected element
    pub fn change_dir_selected(&mut self) {

        // current selected element
        let path = &self.content[self.current];
        set_current_dir(path.as_str());

        // update the content
        self.update();
    

    }

    // sort the files after the input string
    pub fn sort(&mut self, key: String) {

        if key.len() == 0 { return; }
            
        // empty the whole list
        self.content = Vec::new();
        // get all files of the cwd
        let current_filelist = FileList::get_dir();

        // loop and remove the last i characters
        for i in 0..key.len() {
          
            // create new key
            let new_key = &key[0..key.len() - i];
            for n in &current_filelist {
                if n.contains(&new_key.to_lowercase()) || n.contains(&new_key.to_uppercase()) && !self.content.contains(n){
                    self.content.push(n.clone());
                }
            }

        }
         
        if self.content.is_empty() {
             self.content = vec!["Nothing found!".to_string()];
        }

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

impl ScrollableList for FileList {

//    // scrolls up in the list
//    fn scroll_up(&mut self) {
//
//        if self.current != 0 {
//            self.current -= 1;
//        }
//
//    }
//
//    // scrolls down in the list
//    fn scroll_down(&mut self) {
//
//        if self.current != self.content.len() - 1 {
//            self.current += 1;
//        }
//
//    }
//
//    // scrolls to the top of the list
//    fn scroll_top(&mut self) {
//        self.current = 0;
//    }
//
//    // scrolls to the top of the list
//    fn scroll_bottom(&mut self) {
//        self.current = self.content.len();
//    }

    fn get_len(&self) -> usize {
        self.content.len()
    }

    fn get_current(&self) -> usize {
        self.current
    }

    fn set_current(&mut self, new: usize) {
        self.current = new;
    }

    fn items(&self) -> (Vec<String>, usize, Style) {
        (self.content.clone(), self.current, self.highlight)
    }

}
