use std::{
    env::current_dir, 
    fs::copy, 
    fs::create_dir, 
    fs::read_dir, 
    fs::remove_dir_all, 
    fs::remove_file,
    fs::rename,
    path::Path,
};

// action which need some kind of
// input of confirmation, so I have
// to know, which action I have to execute afterwards
pub enum InputConfirmAction {

    Rename,
    Delete,
    Nothing

}

pub struct Action {

    pub clipboard: Vec<String>, // path of a file or directory
    pub status: String,    // status message, display in info widget
    pub current: InputConfirmAction // possible action with an input or confirm step

}

impl Action {

    // create a new action
    pub fn new() -> Self {
        Self {
            clipboard: Vec::new(),
            status: String::new(),
            current: InputConfirmAction::Nothing
        }
    }

    // gets the cwd
    pub fn get_cwd() -> String {
        let cwd = current_dir().expect("Could not get cwd!");
        cwd.to_str().expect("Could not convert to str!").to_string()
    }

    // gets all elements in the cwd
    fn get_dir(path: String) -> Vec<String> {

        read_dir(path.clone())
            .expect(format!("Could not read {}!", path.clone()).as_str())
            .map(|res| {
                res.map(|e| {
                    let mut r = e.path().to_str().unwrap().to_string(); // get the path and put it in a list
                    // removes the annoying "./" before the elements
                    if &r[0..2] == "./" { 
                        r.remove(0);
                        r.remove(0);
                    };
                    r
                })
            })
            .map(|x| x.unwrap()) // gets the actual values
            .collect::<Vec<String>>() // saves them in a Vector of Strings

    }

    // add to vector, if it isn't in it
    fn add_if_not_found(mut vector: Vec<String>, element: String) -> Vec<String> {
        // check if it's found
        match vector.iter().find(|&x| {
            // get the filename
            Path::new(x).file_name()
                .unwrap().to_str().unwrap() == element.as_str()
        }) { // do nothing
            Some(_) => {},
            // add it (the full path)
            None => vector.push(format!("{}/{}", Action::get_cwd(), element))
        } // return the new list
        vector
    }

    // copies a directory recursively
    pub fn copy_recursively(&mut self, base: String, name: String) {

        // the target directory to copy
        let mut n = name.split("/").collect::<Vec<&str>>();
        // remove the first element
        // (it's the same as the targets last dir name)
        n.remove(0);
        let target = format!("{}/{}", base.clone(), n.join("/"));
        // get all the elements of
        // the target directory
        let content = Action::get_dir(target.clone());
        // create the directory to copy to
        if let Err(_) = create_dir(name.clone()) {
            self.status = format!("Could not create directory {}!", name.clone());
            return;
        }

        // loop through all elements
        // and check if they're a dir:
        // - copy recursively again
        // a file:
        // - copy it normal
        for c in content {
            let p = Path::new(&c);
            // get the name
            let c_name = p.file_name()
                .unwrap().to_str().unwrap();
            // check if the element is a
            // directory or a file
            if p.is_dir() {
                // copy the directory recursively
                let new_dir = format!("{}/{}", name, c_name);
                self.copy_recursively(base.clone(), new_dir);
            } else {
                // copy the file normally
                let from = format!("{}/{}", target, c_name);
                let to = format!("{}/{}", name, c_name);
                match copy(from.clone(), to.clone()) {
                    Ok(_) => {},
                    Err(_) => {
                        self.status = format!("Could not copy {}!", from);
                        return;
                    }
                }
            }
        }

    }

    // adds the name to the clipboard
    // and copies the file / directory with the
    // same name to .rufm
    pub fn copy(&mut self, selected: Vec<String>, name: String) {
        self.clipboard = selected;
        // add the current selected element
        // if it isn't already in the list
        self.clipboard = Action::add_if_not_found(self.clipboard.clone(), name);
        // update the status
        self.status = "Copied selected items!".to_string();
    }

    // pastes the clipboard to current location
    pub fn paste(&mut self) {

        if self.clipboard.is_empty() {
            self.status = "Clipboard empty!".to_string();
            return;
        }

        // loop through all elements in the clipboard and paste them
        for c in self.clipboard.clone() {
            // check if the file / directory in the clipboard exists
            let path = Path::new(&c);
            if !path.exists() {
                self.status = "Copied file does not exist anymore!".to_string();
                return;
            } else {
                // get the filename
                let filename = self.check(
                    path.file_name().unwrap()
                        .to_str().unwrap().to_string()
                );
        
                // copy normaly if its a file
                // else recursively
                if path.is_file() {
                    match copy(c.clone(), &filename) {
                        Ok(_) => {},
                        Err(_) => {
                            self.status = "Could not copy the file / directory!".to_string();
                            return;
                        }
                    }
                } else {
                    self.copy_recursively(c, filename.clone());
                }
            }
        }

        // update the status
        self.status = "Pasted clipboard!".to_string();

    }

    // checks if filename exists,
    // adds _copy and restarts
    fn check(&self, name: String) -> String {

        // check if file with similar name already exists
        // read the dir and convert the result a string vector
        let cwd_content = Action::get_dir("./".to_string());
        for c in cwd_content {
            if c == name {
                // rerun the function with " _copy
                return self.check(name.clone() + "_copy");
            }
        }
        // return the name, if
        // it wasn't found in the current directory
        name

    }

    // deletes the specified directory
    pub fn delete(&mut self, selected: Vec<String>, name: String) {

        let mut elements = selected;
        // add the current selected element
        // if it isn't already in the list
        elements = Action::add_if_not_found(elements.clone(), name);

        for c in elements {
            // create path to access information
            let path = Path::new(&c);
            // remove it
            if path.is_dir() {
                match remove_dir_all(path) {
                    Ok(_) => {}
                    Err(_) => {
                        self.status = format!("Failed to delete {}!", path.display());
                        return;
                    }
                }
            } else {
                match remove_file(path) {
                    Ok(_) => {}
                    Err(_) => {
                        self.status = format!("Failed to delete {}!", path.display());
                        return;
                    }
                }
            }
        }
        // update the status
        self.status = "Deleted selected elements!".to_string();

    }

    // renames the element
    pub fn rename(&mut self, name: String, new: String) {
        
        self.status = match rename(name.clone(), new.clone()) {
            Ok(_) => format!("Renamed {} to {}!", name, new),
            Err(_) => format!("Could not rename {} to {}!", name, new)
        }

    }

}
