use std::fs;
use std::{io::Error};

use tui::style::{Color, Style};


pub struct FileList {

    pub content: Vec<String>,
    pub highlightcolor: Style

}

impl FileList {

    pub fn new() -> FileList {

        let cwd_content = fs::read_dir("./")
            .expect("Could not read directory!")
            .map(|res| res.map(|e| e.path().to_str().unwrap().to_string()))
            .collect::<Result<Vec<_>, Error>>().unwrap();

        let style = Style::default().fg(Color::White).bg(Color::Black);


        FileList {

            content: cwd_content,
            highlightcolor: style

        }
    
    }

}
