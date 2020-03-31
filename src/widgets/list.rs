// use fs to access
// the filesystem and read from a directory
use std::{
    io::Error,
    fs::read_dir,
    iter::Iterator
};

// use alloc for Cows
// (Needed for tui::widgets::Text creation)
extern crate alloc;
use alloc::borrow::Cow;

// style and color selected row,
// display text
use tui::style::{Color, Style};
use tui::widgets::Text;


// FileList struct
// Gets used by draw_layout
// to draw the list widget which displays files
pub struct FileList {

    pub current: usize, // current selected item
    pub content: Vec<String>, // all items
    pub highlight: Style // the style, which the current item should have

}

impl FileList {

    // creates a new file list with
    // the content of the current directory
    pub fn new() -> FileList {

        // get all elements off the cwd
        let cwd_content = fs::read_dir("./")
            .expect("Could not read directory!")
            .map(|res| res.map(|e| e.path().to_str().unwrap().to_string()))
            .collect::<Result<Vec<_>, Error>>().unwrap();

        // create the hightlighting style
        let style = Style::default().fg(Color::White).bg(Color::Black);
        
        // return the FileList struct
        FileList {

            current: 0,
            content: cwd_content,
            highlight: style

        }
    
    }

    // scrolls up in the list
    pub fn scroll_up(&mut self) {

        self.current = self.current - 1;

    }

    // scrolls down in the list
    pub fn scroll_down(&mut self) {

        self.current = self.current + 1;

    }

    // 
    pub fn select(&mut self) -> Vec<Text<'_>> {

        self.content.iter().enumerate().map(|(index, f)| {

            if index == self.current {

                Text::Styled(
                    Cow::Borrowed(f),
                    self.highlight
                )

            } else {

                Text::Raw(Cow::Borrowed(f))

            }

        }).collect()

    }

}
