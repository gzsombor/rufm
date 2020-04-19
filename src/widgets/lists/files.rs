// use fs to access
// the filesystem and read from a directory
use std::{
    path::Path,
    fs::read_dir,
    env::var,
    env::current_dir,
    env::set_current_dir,
    process::Command,
    process::Stdio,
    iter::Iterator
};

use tui::style::{ Style, Color };

// import the needed trait
use crate::widgets::traits::CustomList; 

// FileList struct
// Gets used by draw_layout
// to draw the list widget which displays files
pub struct FileList {

    pub selected: Vec<String>, // multiple selected items
    pub current: usize, // current selected item
    pub content: Vec<String>, // all items
    pub key: String, // the search key
    pub sort_style: i8, // the sorting style; 0 = nothing, 1 = search, 2 = abc; 3 = len
    pub border_style: Style, // border colors
    pub open_cmd: Option<String> // the command, which opens a file

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
            }).collect::<Vec<String>>()

    }

    // gets the cwd
    pub fn get_cwd() -> String {
        let cwd = current_dir().expect("Could not get cwd!");
        cwd.to_str().expect("Could not convert to str!").to_string()
    }

    // creates a new file list with
    // the content of the current directory
    pub fn new(bs: [u8; 3], oc: Option<String>) -> Self {

        // get all elements off the cwd
        let cwd_content = Self::get_dir();

        // return the FileList struct
        Self {
    
            selected: Vec::new(),
            current: 0,
            content: cwd_content,
            key: String::new(),
            sort_style: 0,
            border_style: Style::default().fg(Color::Rgb(bs[0], bs[1], bs[2])),
            open_cmd: oc

        }
    
    }

    // gets the current selected element
    pub fn get_current_selected(&self) -> String {
        self.content[self.current].clone()
    }

    // update the list
    pub fn update(&mut self) {
        match self.sort_style {
            1 => { self.sort_len(); },
            2 => { self.sort_search(); },
            _ => { self.sort_default(); }
        }
    }

    // change one directory back
    pub fn change_dir_back(&mut self) {
        // get all elements off the cwd
        set_current_dir("..").expect("Not possible to change back!");
        // clear the selected list
        self.selected = Vec::new();
    }

    // change directory to current selected element
    pub fn change_dir_selected(&mut self) {
        // current selected element
        let path = &self.content[self.current];
        set_current_dir(path.as_str()).expect("Could not change the directory!");
        // clear the selected list
        self.selected = Vec::new();
    }

    // adds the selected element to the list or removes it
    pub fn toggle_select(&mut self) {

        let path = format!("{}/{}", FileList::get_cwd(), self.get_current_selected());
        // check if the element is already in the list
        match self.selected.clone()
            .iter().enumerate().find(|x| x.1 == &path) {
            // if found, remove it 
            Some(v) => { self.selected.remove(v.0); },
            // else add
            None => self.selected.push(path)
        }

    }

    // opens the selected file with the editor
    // specified in $EDITOR
    pub fn open(&self) -> Result<(), &str> {
        let (cmd, args) = match &self.open_cmd {
            Some(v) => {
                // split the commmand by whitspaces
                let parts: Vec<String> = v.split(" ")
                    .map(|x| x.to_string()).collect();
                // get the first element and the arguments
                let cmd = parts.iter().nth(0).expect("Var 'open_cmd' is empty!");
                let mut args = parts[1..parts.len()].to_vec();
                // add the filename
                args.push(self.get_current_selected());
                (cmd.clone(), args)
            },
            None => {
                // get the value of $EDITOR
                let editor = match var("EDITOR") {
                    Ok(v) => v,
                    Err(_) => return Err("No $EDITOR defined!")
                };
                let args = vec![self.get_current_selected()];
                (editor, args)
            }
        };

        // start the editing command 
        // simply editor filename
        let mut edit_cmd = Command::new(cmd.clone());
        edit_cmd.args(args).stdout(Stdio::inherit());
        // run the cmd
        if let Err(_) = edit_cmd.spawn() {
            Err("Editor failed to open!") 
        } else { 
            Ok(())
        }
    }

    // return the list of files with the selected files colored
    pub fn display(&self) -> Vec<String> {

        let mut selected_content = Vec::new();
        // loop through all files and add to the ones
        // in the selected list
        for s in self.content.clone() {
            match self.selected.clone()
                .iter().find(|&x| {
                    let name = Path::new(&x)
                        .file_name().unwrap().to_str().unwrap();
                    name == s.as_str()
                }) {
                // if found, add (selected) to it
                Some(_) => {
                    // create the new text
                    let selected_text = format!("{} (selected)", s);
                    // add element to vec
                    selected_content.push(selected_text);   
                } // dont add something
                None => {
                    selected_content.push(s);
                }
            }
        }

        selected_content 

    }

    // no sorting
    fn sort_default(&mut self) {
        // get the files
        let files = Self::get_dir();
        if files.is_empty() {
            self.content = vec!["Nothing found!".to_string()];
        } else {
            self.content = files;
        }
    }

    // sort the files after the input string
    fn sort_search(&mut self) {

        if self.key.is_empty() { 
            self.sort_default();
            return;
        }
           
        // clear self.content
        self.content = Vec::new();
        // get all files of the cwd
        let current_filelist = Self::get_dir();

        // create new key
        for n in &current_filelist {
            if n.contains(&self.key) { 
                self.content.push(n.clone());
            }
        }
         
        if self.content.is_empty() {
             self.content = vec!["Nothing found!".to_string()];
        }

    }

    // sorts the filelist after length of the name
    fn sort_len(&mut self) {
        // clear self.content
        self.content = Vec::new();
        // get all files of the cwd
        let current_filelist = Self::get_dir();

        for f in current_filelist {
            let lf = f.len();
            let mut insert = false;
            let mut pos = 0;
            for (i, s) in self.content.iter().enumerate() {
                let ls = s.len();
                if lf < ls && !insert {
                    pos = i; 
                    insert = true; 
                }
            }
            if insert {
                self.content.insert(pos, f); 
            } else { 
                self.content.push(f);
           
            }
        }

        if self.content.is_empty() {
             self.content = vec!["Nothing found!".to_string()];
        }
    }

    // switches between the 3 avaible sorting styles
    pub fn toggle_sort_style(&mut self) {
        if self.sort_style == 1 {
            self.sort_style = 0;
        } else {
            self.sort_style += 1;
        }
    }

}

impl CustomList for FileList {

    fn get_len(&self) -> usize {
        self.content.len()
    }

    fn get_current(&self) -> usize {
        self.current
    }

    fn set_current(&mut self, new: usize) {
        self.current = new;
    }

}
