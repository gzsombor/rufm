// cmd line arguments
use std::env::{
    args, 
    set_current_dir
};

use std::process::exit;
use std::path::Path;

pub struct Options {
    pub config: String
}

impl Options {

    // create a new Options struct
    pub fn new() -> Self {
        Self {
            // default path
            config: "~/.config/rufm/config.ini".to_string()
        }
    }

    // evaluate cmd arguments
    pub fn eval(&mut self) {

        // get the cmd arguments
        let args: Vec<String> = args().collect();
        let mut args = args[1..].iter();

        // match arguments
        // and execute function
        loop {
            // get the next argument
            let arg = args.next();
            match arg {
                // if its found, match it
                Some(a) => match a.as_str() {
                    // help menu
                    "-h" | "--help" => self.help(),
                    // custom directory
                    "-d" => {
                        let next_arg = args.next();
                        match next_arg {
                            Some(v) => self.change(v.clone()),
                            None => self.help(),
                        }
                    },
                    // custom path to config
                    "-c" => {
                        let next_arg = args.next();
                        match next_arg {
                            Some(v) => self.config(v.clone()), 
                            None => self.help()
                        }
                    },
                    _ => {}
                },
                // else, stop the function
                None => break,
            }
        }

    }

    // help menu
    fn help(&self) {
        println!("
\nRufm - A rustical file manager
-------------------------------------------\n
Use -h | --help   to display this help menu
Use -d <path>     to change the directory
Use -c <path>     to change the path to the config file\n
");
        exit(1);
    }

    // changes to target directory
    fn change(&self, target: String) {
        match set_current_dir(target.clone()) {
            Ok(_) => {}
            Err(_) => {
                println!("Could not change to {}, aborting ...", target);
                exit(1);
            }
        }
    }

    // sets new path for config file
    fn config(&mut self, target: String) {
        let p = Path::new(&target);
        if p.is_file() {
            self.config = target;
        } else {
            println!("No such file: {}", target);
            exit(1);
        }
    }

}
