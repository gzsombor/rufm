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

    pub clipboard: String, // path of a file or directory
    pub status: String,    // status message, display in info widget
    pub current: InputConfirmAction // possible action with an input or confirm step

}

impl Action {

    // create a new action
    pub fn new() -> Self {
        Self {
            clipboard: String::new(),
            status: String::new(),
            current: InputConfirmAction::Nothing
        }
    }

    // gets the cwd
    fn get_cwd() -> String {
        let cwd = current_dir().expect("Could not get cwd!");
        cwd.to_str().expect("Could not convert to str!").to_string()
    }

    // gets all elements in the cwd
    fn get_dir(path: String) -> Vec<String> {

        read_dir(path)
            .expect("Could not read directory!")
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

    // copies a directory recursively
    pub fn copy_recursively(&self, name: String) {

        // the target directory to copy
        let mut n = name.split("/").collect::<Vec<&str>>();
        let n = &n[1..n.len()];
        let target = format!("{}/{}", self.clipboard.clone(), n.join("/"));
        // get all the elements of
        // the target directory
        let content = Action::get_dir(target.clone());
        // create the directory to copy to
        create_dir(name.clone());

        // loop through all elements
        // and check if they're a dir:
        // - copy recursively again
        // a file:
        // - copy it normal
        for c in content {
            // get the name
            let c_name = &c
                .split("/")
                .collect::<Vec<&str>>()
                .pop()
                .unwrap()
                .to_string();
            // check if the element is a
            // directory or a file
            let p = Path::new(&c);
            if p.is_dir() {
                // copy the directory recursively
                let new_dir = format!("{}/{}", name.clone(), c_name);
                self.copy_recursively(new_dir);
            } else {
                // copy the file normally
                let from = format!("{}/{}", target.clone(), c_name);
                let to = format!("{}/{}", name.clone(), c_name);
                copy(from, to).expect("Could not copy the directory!");
            }
        }


    }

    // adds the name to the clipboard
    // and copies the file / directory with the
    // same name to .rufm
    pub fn copy(&mut self, name: String) {
        let cwd = Action::get_cwd();
        self.clipboard = format!("{}/{}", cwd, name);
        // update the status
        self.status = format!("Copied {}!", name);
    }

    // pastes the clipboard to current location
    pub fn paste(&mut self) {

        // get the filename
        let filename = self.check(
            self.clipboard
                .split("/")
                .collect::<Vec<&str>>()
                .clone()
                .pop()
                .expect("Could not pop last element!")
                .to_string()
        );
        // copy normaly if its a file
        // else recursively
        let p = &self.clipboard.clone();
        let p = Path::new(p);
        if p.is_file() {
            copy(self.clipboard.clone(), &filename).expect("Could not copy the file / directory!");
        } else {
            self.copy_recursively(filename.clone());
        }
        // update the status
        self.status = format!("Pasted {}!", &filename);

    }

    // checks if filename exists,
    // adds _copy and restarts
    fn check(&self, name: String) -> String {

        // check if file with similar name already exists
        // read the dir and convert the result a string vector
        let cwd_content = Action::get_dir("./".to_string());
        for c in cwd_content {
            if c == name {
                return self.check(name.clone() + "_copy");
            }
        }
        // return the name, if
        // it wasn't found in the current directory
        name

    }

    // deletes the specified directory
    pub fn delete(&mut self, name: String) {

        // create path to access information
        let path = Path::new(&name);
        // remove it
        if path.is_dir() {
            match remove_dir_all(path) {
                Ok(_) => {}
                Err(_) => self.status = format!("Failed to delete {}!", name),
            }
        } else {
            match remove_file(path) {
                Ok(_) => {}
                Err(_) => self.status = format!("Failed to delete {}!", name),
            }
        }
        // update the status
        self.status = format!("Deleted {}!", name);

    }

    // renames the element
    pub fn rename(&mut self, name: String, new: String) {
        
        rename(name.clone(), new.clone());
        self.status = format!("Renamed {} to {}!", name, new);

    }
    
}
