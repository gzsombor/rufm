// import the needed trait
use crate::widgets::traits::{
    CustomParagraph,
    EditableParagraph
};

use std::{
    fs,
    collections::HashMap,
    os::unix::fs::MetadataExt
};

use tui::style::{ Style, Color };

// actions which need some kind of
// input of confirmation, so I have
// to know, which action I have to execute afterwards
#[derive(Copy, Clone)]
pub enum Input {
    Rename,
    Command
}

#[derive(Copy, Clone)]
pub enum Confirm {
    Delete
}

#[derive(Clone)]
pub enum InfoMode {

    Status,
    Information,
    Input(Input),
    Confirm(Confirm)

}

pub struct Info {

    pub content: String,
    pub mode: InfoMode, // the current mode
    pub border_style: Style // border colors

}

impl Info {

    // create a new info widget
    pub fn new(bs: [u8; 3]) -> Self {
        Self {
            content: String::new(),
            mode: InfoMode::Information,
            border_style: Style::default().fg(Color::Rgb(bs[0], bs[1], bs[2]))
        }
    }

    // get the permissions and convert them
    // to a nice looking string like rwxr-xr--
    fn get_permissions(md: fs::Metadata) -> String {

        // the kind of file
        let kind = if md.is_dir() { "d" } else { "-" };
        // get the permissions and 
        // convert the to octal numbers + get only the last three
        // => looks like 755 or something
        let mode = md.mode();
        let mode = format!("{:o}", mode);
        // hashmap of permission-numbders
        // and their corresponding string
        let mut permissions = HashMap::new();
        permissions.insert("0", "---");
        permissions.insert("1", "--x");
        permissions.insert("2", "-w-");
        permissions.insert("3", "-wx");
        permissions.insert("4", "r--");
        permissions.insert("5", "r-x");
        permissions.insert("6", "rw-");
        permissions.insert("7", "rwx");
        
        // get the actual permissions
        // from 100xyz to xyz (these are the only important bits)
        //
        // convert the string to a list of chars
        let modes_list: Vec<&str> = mode
            .split("").collect();
        // get the last four except the last
        // explanation: if string is split with an empty seperator,
        // rust adds an empty string in the front and back
        let mode = &modes_list[modes_list.len() - 4..modes_list.len() - 1];
        let mut file_permission = String::from(kind);
        // match the permissions to the string bits
        for m in mode {
            if let Some(v) = permissions.get(m) {
                // add the according string to the permissions
                file_permission.push_str(v);
            }
        }

        file_permission

    }

    // update the content with information on file
    pub fn update(&mut self, name: String) {

        match self.mode {

            InfoMode::Information => match fs::metadata(name) {
                Ok(md) => {
                    // get the size of the file
                    let len = md.size();
                    let file_permission = Info::get_permissions(md); 
                    // update the content var
                    self.content = format!("{} {:>6}B", file_permission, len);
                },

                Err(_) => {
                    self.content = "No information avaible!".to_string();
                }
            },

            InfoMode::Status => self.mode = InfoMode::Information,

            _ => {}

        }

    }

    // match the mode and
    // return the right title
    pub fn get_title(&self) -> &str {
        // match the mode (InfoMode enum)
        let output = match self.mode {
            InfoMode::Status => " Status ",
            InfoMode::Information => " Info ",
            InfoMode::Input(v) => match v {
                Input::Rename => " Rename ",
                Input::Command => " Command "
            },
            InfoMode::Confirm(v) => match v {
                Confirm::Delete => " Delete "
            }
        }; return output
    }


}

impl CustomParagraph for Info {

    fn items(&self) -> String {
        self.content.clone()
    }

}

impl EditableParagraph for Info {

    // cuts off the explanation
    // like a special char or a word like "Name: " at the beginning
    fn get_content(&self) -> String {
        self.content.clone()
    }

    fn set_content(&mut self, new: String) {
        self.content = new;
    }

}
