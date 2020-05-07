// import the needed trait
use crate::widgets::traits::{CustomParagraph, EditableParagraph};

use tui::style::{Color, Style};

// actions which need some kind of
// input of confirmation, so I have
// to know, which action I have to execute afterwards
#[derive(Copy, Clone)]
pub enum Input {
    Search,
    Rename,
    Command,
}

#[derive(Copy, Clone)]
pub enum Confirm {
    Delete,
}

#[derive(Clone)]
pub enum SearchMode {
    Input(Input),
    Confirm(Confirm),
}

pub struct Search {
    pub content: String,
    pub mode: SearchMode, // the current mode
    pub border_style: Style, // border colors
}

impl Search {
    // create a new widget
    pub fn new(bs: [u8; 3]) -> Search {
        Search {
            content: String::new(),
            mode: SearchMode::Input(Input::Search),
            border_style: Style::default().fg(Color::Rgb(bs[0], bs[1], bs[2])),
        }
    }
    // match the mode and
    // return the right title
    pub fn get_title(&self) -> &str {
        // match the mode (InfoMode enum)
        // spaces just look cooler
        let output = match self.mode {
            SearchMode::Input(v) => match v {
                Input::Search => " Search ",
                Input::Rename => " Rename ",
                Input::Command => " Command ",
            },
            SearchMode::Confirm(v) => match v {
                Confirm::Delete => " Delete ",
            },
        };
        output
    }
}

impl CustomParagraph for Search {
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
