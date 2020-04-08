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

    // gets all elements in the cwd
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
            self.clipboard.split("/")
                .collect::<Vec<&str>>().clone().pop()
                .expect("Could not pop last element!").to_string()
        );
        // copy the file
        copy(self.clipboard.clone(), &filename).expect("Could not copy the file / directory!");  
        // update the status
        self.status = format!("Pasted {}!", &filename);
    }

    // checks if filename exists,
    // adds _copy and restarts
    fn check(&self, name: String) -> String {
        // check if file with similar name already exists
        // read the dir and convert the result a string vector
        let cwd_content = Action::get_dir();
        for c in cwd_content {
            println!("{}", c);
            if c == name {
                println!("Match!");
                return self.check(name + "_copy");
            }
        }; name 
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
