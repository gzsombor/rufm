// cmd line arguments
use std::env::{
    args, 
    set_current_dir
};

use std::process::exit;

// evaluate cmd arguments
pub fn eval() {
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
                "-h" | "--help" => help(),
                // custom directory
                "-d" => {
                    let next_arg = args.next();
                    match next_arg {
                        Some(v) => change(v.clone()),
                        None => help(),
                    }
                }
                _ => {}
            },
            // else, stop the function
            None => break,
        }
    }
}

// help menu
fn help() {
    println!("\nRufm - A rustical file manager");
    println!("-------------------------------------------\n");
    println!("Use -h | --help   to display this help menu");
    println!("Use -d            to change the directory");
    exit(1);
}

// changes to target directory
fn change(target: String) {
    match set_current_dir(target.clone()) {
        Ok(_) => {}
        Err(_) => {
            println!("\nCould not change to {}, aborting ...", target);
            exit(1);
        }
    }
}
