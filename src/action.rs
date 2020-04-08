use std::{
    path::Path,
    fs::remove_file,
    fs::remove_dir_all,
    env::current_dir
};


pub struct Action {

    pub clipboard: String, // path of a file or directory
    pub status: String // status message, display in info widget

}

// add action methods
impl Action {

    // create a new action
    pub fn new() -> Self {

        Self {
            clipboard: String::new(),
            status: String::new()
        }

    }

    // gets the cwd
    fn get_cwd() -> String {
        let cwd = current_dir().expect("Could not get cwd!");
        cwd.to_str().expect("Could not convert to str!").to_string()
    }
    
    // adds the name to the clipboard
    // and copies the file / directory with the
    // same name to .rufm
    pub fn copy(&mut self, name: String) {
        let cwd = Action::get_cwd();         
        self.clipboard = format!("{}/{}", cwd, name);
    }
   
    // pastes the clipboard to current location
    pub fn paste(&self) {
        // let cwd = Action::get_cwd();
    }

    // deletes the specified directory
    pub fn delete(&mut self, name: String) {
        // create path to access information    
        let path = Path::new(&name); 
        // remove it
        if path.is_dir() {
            match remove_dir_all(path) {
                Ok(_) => {},
                Err(_) => self.status = format!("Failed to delete {}!", name)
            }
        } else {
            match remove_file(path) {
                Ok(_) => {},
                Err(_) => self.status = format!("Failed to delete {}!", name)
            }
        }
        // update the status
        self.status = format!("Deleted {}!", name);
    }

}
