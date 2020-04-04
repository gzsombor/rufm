// use fs to access
// the filesystem and read from a directory
use std::{
    env::set_current_dir,
    env::current_dir, iter::Iterator
};

// import the needed trait
use crate::widgets::traits::ScrollableList; 

// style and color selected row,
// display text
use tui::style::{Color, Style};

// FileList struct
// Gets used by draw_layout
// to draw the list widget which displays files
pub struct Favourites {

    pub current: usize, // current selected item
    pub names: Vec<String>, // all names
    pub paths: Vec<String>, // all paths (same index as names)
    pub highlight: Style // the style, which the current item should have

}

impl Favourites {

    // creates a new file list with
    // the content of the current directory
    pub fn new() -> Favourites {

        // get all elements off the cwd
        let favs_names = vec!["Home", "Dotfiles"]
            .iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let favs_paths = vec!["/home/boss", "/home/boss/dotfiles"]
            .iter().map(|x| x.to_string()).collect::<Vec<String>>();


        // create the hightlighting style
        let style = Style::default().fg(Color::White).bg(Color::Blue);
        
        // return the FileList struct
        Favourites {

            current: 0,
            names: favs_names,
            paths: favs_paths,
            highlight: style

        }
    
    }

    // change directory to current selected element
    pub fn change_dir_selected(&mut self) {

        // current selected element
        let path = &self.paths[self.current];
        set_current_dir(path.as_str()); // path.as_str());

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

impl ScrollableList for Favourites {
 
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
//        if self.current != self.names.len() - 1 {
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
//        self.current = self.names.len() - 1;
//    }

    fn get_len(&self) -> usize {
        self.names.len()
    }

    fn get_current(&self) -> usize {
        self.current
    }

    fn set_current(&mut self, new: usize) {
        self.current = new;
    }

    fn items(&self) -> (Vec<String>, usize, Style) {
        (self.names.clone(), self.current, self.highlight)
    }

}
