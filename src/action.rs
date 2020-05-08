use std::{
    env::current_dir, fs::copy, fs::create_dir, fs::read_dir, fs::remove_dir_all, fs::remove_file,
    fs::rename, path::Path, process::Command, str,
};

pub struct Action {
    pub clipboard: Vec<String>, // path of a file or directory
    pub status: String,         // status message, display in info widget
}

impl Action {
    // create a new action
    pub fn new() -> Self {
        Self {
            clipboard: Vec::new(),
            status: String::new(),
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
            .expect(format!("Could not read {}!", path).as_str())
            .map(|res| {
                res.map(|e| {
                    // get the path and put it in a list
                    let mut r = e.path().to_str().unwrap().to_string();
                    // removes the annoying "./" before the elements
                    if &r[0..2] == "./" {
                        r.remove(0);
                        r.remove(0);
                    };
                    return r;
                })
            })
            .map(|x| x.unwrap()) // gets the actual values
            .collect::<Vec<String>>() // saves them in a Vector of Strings
    }

    // add to vector, if it isn't in it
    fn add_if_not_found(mut vector: Vec<String>, element: String) -> Vec<String> {
        // check if it's found
        if vector.iter().find(|&x| {
            // get the filename
            Path::new(x).file_name().unwrap().to_str().unwrap() == element.as_str()
        }).is_none() {
            // add it (the full path)
            vector.push(format!("{}/{}", Action::get_cwd(), element));
        } // return the new list
        vector
    }

    // copies a directory recursively
    pub fn copy_recursively(&mut self, base: String, name: String) {
        // the target directory to copy
        let mut n = name.split('/').collect::<Vec<&str>>();
        // remove the first element
        // (it's the same as the targets last dir name)
        n.remove(0);
        let target = format!("{}/{}", base.clone(), n.join("/"));
        // get all the elements of
        // the target directory
        let content = Action::get_dir(target.clone());
        // create the directory to copy to
        if create_dir(name.clone()).is_err() {
            self.status = format!("Failed to copy {}!", name);
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
            let c_name = p.file_name().unwrap().to_str().unwrap();
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
                if copy(from.clone(), to.clone()).is_err() {
                    self.status = format!("Failed to copy {}!", name);
                    return;
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
        self.status = "Copied the selected elements!".to_string();
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
                self.status = format!("File {} not found!", path.display());
                return;
            } else {
                // get the filename
                let filename = self.check(path.file_name().unwrap().to_str().unwrap().to_string());

                // copy normaly if its a file
                // else recursively
                if path.is_file() {
                    // check if successfull
                    if copy(c.clone(), &filename).is_err() {
                        self.status = format!("Failed to copy {}!", c);
                        return;
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
                return self.check(name + "_copy");
            }
        }
        // return the name, if
        // it wasn't found in the current directory
        name
    }

    // deletes the specified directory
    pub fn delete(&mut self, selected: Vec<String>, name: String) {
        // add the current selected element
        // if it isn't already in the list
        let elements = Action::add_if_not_found(selected, name);

        for c in elements {
            // create path to access information
            let path = Path::new(&c);
            // remove it
            if path.is_dir() {
                if remove_dir_all(path).is_err() { 
                    self.status = format!("Failed to remove {}!", path.display());
                    return;
                }
            } else if remove_file(path).is_err() {
                self.status = format!("Failed to remove {}!", path.display());
                return;
            }
        }
        // update the status
        self.status = "Deleted files!".to_string();
    }

    // renames the element
    pub fn rename(&mut self, name: String, new: String) {
        // try to rename the file
        self.status = match rename(name.clone(), new.clone()) {
            // update the status accordingly
            Ok(_) => format!("File renamed to {}!", new),
            Err(_) => format!("Failed to rename {}!", name)
        }
    }

    // runs the inputed command
    pub fn run_cmd(&mut self, input: String) {
        // get the actual cmd (first element)
        let cmd_list: Vec<&str> = input.split(' ').collect();
        let first = match cmd_list.get(0) {
            Some(v) => v,
            None => {
                self.status = "No input!".to_string();
                return;
            }
        };
        // create the command
        let mut cmd = Command::new(first);
        if cmd_list.len() > 1 {
            let args = &cmd_list[1..cmd_list.len()];
            // add the arguments
            cmd.args(args);
        }
        // run the command
        // update the status
        self.status = match cmd.output() {
            Ok(v) => {
                // check if command returned output
                let output = str::from_utf8(&v.stdout).expect("Could not decode terminal output!");
                if output == "" {
                    "Success => no output".to_string()
                } else {
                    format!("Success => {}", output)
                }
            }
            Err(_) => "Command failed!".to_string(),
        };
    }
}
