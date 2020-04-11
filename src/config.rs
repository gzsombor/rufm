// use to decode the toml
use serde_derive::Deserialize;

// use to read the file
use std::{
    env::var,
    fs::File,
    io::prelude::*
};


// Structs, where the toml
// config will get parse into
#[derive(Deserialize)]
pub struct Config {

    pub colors: Colors,
    pub favourites: Favourites

}

#[derive(Deserialize)]
pub struct Favourites {

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


impl Config {

    // default configuration
    pub fn default() -> Self {

        Self {
            colors: Colors {
                border_normal: [255, 255, 255],
                border_highlight: [158, 232, 255],
                text_highlight: Color {
                    fg: Some([158, 232, 255]),
                    bg: None
                }
            },
            favourites: Favourites {
                names: vec!["Root".to_string()],
                paths: vec!["/".to_string()]
            }
        }

    }

}


pub fn create_config(filename: String) -> Config {
    
    // read from the file
    let mut content = String::new();
    // get the home directory
    let home = var("HOME").expect("Could not get $HOME!");
    let filename = filename.replace("~", home.as_str().clone());

    match File::open(&filename) {

        Ok(v) => {

            // if it exists, assign it
            let mut config_file = v;
            // read it
            config_file.read_to_string(&mut content).expect("Could not read the config file!");

            // parse the variable to the Config struct
            let mut config: Config = toml::from_str(&content).expect("Could not parse toml!");
            // replace all ~ with $HOME var
            config.favourites.paths = config.favourites.paths.iter()
                .map(|x| x.replace("~", home.as_str().clone())).collect();
            
            config

        }

        Err(_) => {
            // else panic, because everything depends on the configuration
            println!("Could not read configuration file at '$HOME/.config/rufm/config.ini'! Using default configuration ...");
            Config::default()
        }

    }

}