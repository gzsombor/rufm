// use fs to access
// the filesystem and read from a directory
use std::{
    env::current_dir, env::set_current_dir, env::var, fs::read_dir, iter::Iterator, path::Path,
    process::Command, process::Stdio,
};

use tui::style::{Color, Style};

// import the needed trait
use crate::widgets::traits::CustomList;

// different styles of sorting
pub enum SortStyles {
    Len,
    Alphabet,
    Search,
    Normal,
}

// FileList struct
// Gets used by draw_layout
// to draw the list widget which displays files
pub struct FileList {
    pub selected: Vec<String>,  // multiple selected items
    pub current: usize,         // current selected item
    pub content: Vec<String>,   // all items
    pub key: String,            // the search key
    pub sort_style: SortStyles, // the sorting style; 0 = nothing, 1 = search, 2 = abc; 3 = len
    pub border_style: Style,    // border colors
    pub open_cmd: String,       // the command, which opens a file
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
                    x.remove(0);
                    x.remove(0); // remove the ./ prefix
                };
                x
            })
            .collect::<Vec<String>>()
    }

    // gets the cwd
    pub fn get_cwd() -> String {
        let cwd = current_dir().expect("Could not get cwd!");
        cwd.to_str().expect("Could not convert to str!").to_string()
    }

    // creates a new file list with
    // the content of the current directory
    pub fn new(bs: [u8; 3], oc: String) -> Self {
        // get all elements off the cwd
        let cwd_content = Self::get_dir();

        // return the FileList struct
        Self {
            selected: Vec::new(),
            current: 0,
            content: cwd_content,
            key: String::new(),
            sort_style: SortStyles::Normal,
            border_style: Style::default().fg(Color::Rgb(bs[0], bs[1], bs[2])),
            open_cmd: oc,
        }
    }

    // gets the current selected element
    pub fn get_current_selected(&self) -> String {
        self.content[self.current].clone()
    }

    // update the list
    pub fn update(&mut self) {
        match self.sort_style {
            SortStyles::Len => self.sort_len(),
            SortStyles::Search => self.sort_search(),
            SortStyles::Alphabet => self.sort_alphabet(),
            SortStyles::Normal => self.sort_default(),
        }
    }

    // change one directory back
    pub fn change_dir_back(&mut self) -> Result<(), String> {
        match set_current_dir("..") {
            Ok(_) => {
                // clear the selected list
                self.selected = Vec::new();
                Ok(())
            }
            Err(_) => Err("Could not change back!".to_string())
        }
    }

    // change directory to current selected element
    pub fn change_dir_selected(&mut self) -> Result<(), String> {
        // current selected element
        let path = &self.content[self.current];
        match set_current_dir(path.as_str()) {
            Ok(_) => {
                // clear the selected list
                self.selected = Vec::new();
                Ok(())
            }
            Err(_) => Err(format!("Could not change to {}!", path)),
        }
    }

    // adds the selected element to the list or removes it
    pub fn toggle_select(&mut self) {
        let path = format!("{}/{}", FileList::get_cwd(), self.get_current_selected());
        // check if the element is already in the list
        match self
            .selected
            .clone()
            .iter()
            .enumerate()
            .find(|x| x.1 == &path)
        {
            // if found, remove it
            Some(v) => {
                self.selected.remove(v.0);
            }
            // else add
            None => self.selected.push(path),
        }
    }

    // opens the selected file with the editor
    // specified in $EDITOR
    pub fn open(&mut self) -> Result<(), &str> {
        // check if selected element is a file or a directory
        let current_selected = self.get_current_selected();

        // split the commmand by whitspaces
        let mut parts: Vec<String> = self.open_cmd.split(' ').map(|x| x.to_string()).collect();

        // substitute variables if found
        for p in &mut parts {
            if &*p.as_str() == "$EDITOR" {
                let editor = match var("EDITOR") {
                    Ok(v) => v,
                    Err(_) => return Err("No $EDITOR defined!"),
                }; // update the value
                *p = editor;
            }
        }

        // get the first element and the arguments
        let cmd = parts.get(0).expect("Var 'open_cmd' is empty!");
        let mut args = parts[1..parts.len()].to_vec();
        // add the filename
        args.push(current_selected);

        // start the editing command
        // simply editor filename
        let mut edit_cmd = Command::new(cmd.clone());
        edit_cmd.args(args).stdout(Stdio::inherit());
        // run the cmd
        if edit_cmd.spawn().is_err() {
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
            match self.selected.clone().iter().find(|&x| {
                let name = Path::new(&x).file_name().unwrap().to_str().unwrap();
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

    // function to check if the
    // current place is right
    fn find_place(i: usize, ins_elem: String, sor_elem: String) -> bool {
        // convert the to lowercase
        let sor_elem = sor_elem.to_lowercase();
        let ins_elem = ins_elem.to_lowercase();
        // check if s is
        // actually something
        // get the unicode values
        let sor_char = match sor_elem.as_bytes().iter().nth(i) {
            Some(v) => v,
            None => return false,
        };
        let ins_char = match ins_elem.as_bytes().iter().nth(i) {
            Some(v) => v,
            None => return true,
        }; // function to check if unicode
           // points to a special char
        let spec = |x: &u8| -> bool {
            x < &65 || x > &122
        }; // check if the sorted element
           // or inserted elemnt is a special char
        if spec(ins_char) && !spec(sor_char) {
            return false;
        } else if !spec(ins_char) && spec(sor_char) {
            return true;
        } else if spec(ins_char) && spec(sor_char) {
            return Self::find_place(i + 1, ins_elem, sor_elem);
        } // check if the place is right
          // unicode val smaller
          // else continue to the
          // next one
        match ins_char {
            v if v < sor_char => return true,
            v if v == sor_char => {
                // recoursive (check for next index)
                return Self::find_place(i + 1, ins_elem, sor_elem);
            }
            // if nothing was found
            // just return false
            _ => return false
        }
    }

    // sorts after the alphabet
    // works with comparing the unicode values
    fn sort_alphabet(&mut self) {
        // clear the content
        self.content = Vec::new();
        // get all the files of the cwd
        let current_filelist = Self::get_dir();
        // loop through all the files and find
        // the right position for them
        for w in current_filelist.iter() {
            // the position to insert the element
            let mut pos = None;
            // get the place to insert the element
            // with looping through the already sorted words
            for (i, s) in self.content.iter().enumerate() {
                // check if position was already assigned
                if pos.is_none() {
                    // if place is found assign it
                    if Self::find_place(0, w.to_string(), s.to_string()) {
                        pos = Some(i)
                    }
                }
            } // insert or append the element
            match pos {
                // else insert a the given place
                Some(v) => self.content.insert(v, w.to_string()),
                // append if no place found
                None => self.content.push(w.to_string()),
            }
        }
    }

    // switches between the 3 avaible sorting styles
    pub fn toggle_sort_style(&mut self) {
        match self.sort_style {
            // switch between the two
            SortStyles::Normal => self.sort_style = SortStyles::Len,
            SortStyles::Len => self.sort_style = SortStyles::Alphabet,
            SortStyles::Alphabet => self.sort_style = SortStyles::Normal,
            // else do nothing
            _ => {}
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
