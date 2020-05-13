// cmd line arguments
// every arguments gets handled by
// the eval method and return a bool
// this bool indicated is the program should
// exit or continue
use std::env::{args, set_current_dir};

use std::path::Path;
use std::process::exit;

// possible cmdline options
pub struct Options {
    pub config: Option<String>,
}

impl Options {
    // create a new Options struct
    pub fn new() -> Self {
        Self {
            // default path
            config: None,
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
            let stop = match arg {
                // if its found, match it
                Some(a) => match a.as_str() {
                    // help menu
                    "-h" | "--help" => self.help(None),
                    // custom directory
                    "-d" | "--directory" => {
                        let next_arg = args.next();
                        match next_arg {
                            Some(v) => self.change(v.clone()),
                            None => self.help(Some("No directory specified!")),
                        }
                    }
                    // custom path to config
                    "-c" | "--config" => {
                        let next_arg = args.next();
                        match next_arg {
                            Some(v) => self.config(v.clone()),
                            None => self.help(Some("No configuration file specified!")),
                        }
                    }
                    a => self.help(Some(format!("No such option: {}", a).as_str())),
                },
                // else, stop the function
                None => break,
            };
            // check if program should stop
            if stop {
                exit(1);
            }
        }
    }

    // help menu
    fn help(&self, failmsg: Option<&str>) -> bool {
        let help_menu = String::from(
"
Rufm - A file manager written in Rust
-------------------------------------

Usage: 
    rufm [options]

Options:
    -h or --help                 display this help menu
    -d or --directory <path>     change the directory to <path>
    -c or --config <path>        use the config file at <path>
"
        );
        // check if a fail message
        // is provided => print it
        if let Some(msg) = failmsg {
            println!("\n{}\nUse --help for the help menu!", msg);
            return true
        }
        // print the real help menu
        println!("{}", help_menu);
        true
    }

    // changes to target directory
    fn change(&self, target: String) -> bool {
        if set_current_dir(target.clone()).is_err() {
            println!("Could not change to {}, aborting ...", target);
            return true
        }
        false
    }

    // sets new path for config file
    fn config(&mut self, target: String) -> bool {
        let p = Path::new(&target);
        if p.is_file() {
            self.config = Some(target);
            false
        } else {
            println!("No such file: {}", target);
            true
        }
    }
}
