// use to decode the toml
use toml::Value;
use serde_derive::Deserialize;

// use to read the file
use std::{
    env::var,
    fs::File,
    path::Path,
    io::prelude::*
};

#[derive(Deserialize)]
pub struct Config {

    pub colors: Colors,
    pub favourites: Favs

}

#[derive(Deserialize)]
pub struct Favs {

    pub names: Vec<String>,
    pub paths: Vec<String>

}

#[derive(Deserialize)]
pub struct Colors {

    pub border_normal: [u8; 3],
    pub border_highlight: [u8; 3],

    pub text_highlight: Color 

}

#[derive(Deserialize)]
pub struct Color {

    pub fg: Option<[u8; 3]>,
    pub bg: Option<[u8; 3]>,

}

pub fn create_config() -> Config {
    
    // read from the file
    let mut content = String::new();
    // get the home directory
    let home = var("HOME").expect("Could not get $HOME");
    // compose the filename
    let filename = String::from(home + "/.config/rufm/config.ini");
    match File::open(&filename) {
        Ok(v) => {
            // if it exists, assign it
            let mut config_file = v;
            // read it
            config_file.read_to_string(&mut content);
            // parse the variable to the Config struct
            let config: Config = toml::from_str(&content).expect("Could not parse toml!");
            // return the config
            config
        }
        Err(_) => {
            // else panic, because everything depends on the configuration
            panic!("Could not read configuration file at '$HOME/.config/rufm/config.ini'!");
        }
    }

}
