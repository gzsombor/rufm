pub struct Action {

    pub clipboard: String // path of a file or directory

}

// add action methods
impl Action {

    // create a new action
    pub fn new() -> Self {

        Self {
            clipboard: String::new()
        }

    }

    // adds the name to the clipboard
    // and copies the file / directory with the
    // same name to .rufm
    pub fn copy(&mut self, name: String) {
        self.clipboard = name;
    }
   
    // pastes the clipboard to current location

}
