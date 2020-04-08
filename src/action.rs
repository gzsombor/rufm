use std::{
    path::Path,

    fs::copy,
    fs::read_dir,
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
        // get the filename
        let filename = self.check(
            self.clipboard.split("/")
                .collect::<Vec<&str>>().clone().pop()
                .expect("Could not pop last element!").to_string()
        );
        // copy the file
        copy(self.clipboard.clone(), filename);  
    }

    // checks if filename exists,
    // adds _copy and restarts
    fn check(&self, name: String) -> String {
        // check if file with similar name already exists
        let cwd_content: Vec<String> = read_dir("./")
            .expect("Could not read the directory!")
            .map(|x| x.expect("Could not read the directory!").path().to_str()
                 .expect("Could not read the directory!").to_string()).collect();
        for c in cwd_content {
            if c == name {
                return self.check(name + "_copy");
            }
        }
        name 
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
