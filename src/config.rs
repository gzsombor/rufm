// use to decode the toml
use serde_derive::Deserialize;

// use to read the file
use std::{ env::var, fs::File, io::prelude::Read
};


// Structs, where the toml
// config will get parse into
#[derive(Deserialize)]
pub struct Config {

    pub keys: Keys,
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

#[derive(Deserialize)]
pub struct Keys {
    
    pub rename: Option<String>,
    pub copy: Option<String>,
    pub paste: Option<String>,
    pub delete: Option<String>,
    pub search: Option<String>,
    pub sort: Option<String>,
    pub favourites: Option<String>

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
            },

            keys: Keys {
                rename: Some(String::from("R")),
                copy: Some(String::from("C")),
                paste: Some(String::from("P")),
                delete: Some(String::from("D")),
                search: Some(String::from("/")),
                sort: Some(String::from("\t")),
                favourites: Some(String::from("F"))
            }

        }

    }

}

fn change_default_keys(mut keys: Keys) -> Keys {

    // default key - value pairs
    let default_keys = [
      ("rename", "R"),
      ("copy", "C"),
      ("paste", "P"),
      ("delete", "D"),
      ("search", "/"),
      ("sort", "\t"),
      ("favourites", "F")
    ];
  
    // match all values and replace
    // the keys if they're None values
    // they're probably is a better way, let me know
    for k in &default_keys {

        match k.0 {

            "rename" => match keys.rename {
                Some(_) => {},
                None => keys.rename = Some(k.1.to_string())
            },

            "copy" => match keys.copy {
                Some(_) => {},
                None => keys.copy = Some(k.1.to_string())
            },

            "paste" => match keys.paste {
                Some(_) => {},
                None => keys.paste = Some(k.1.to_string())
            },                   

            "delete" => match keys.delete {
                Some(_) => {},
                None => keys.delete = Some(k.1.to_string())
            },                   

            "search" => match keys.search {
                Some(_) => {},
                None => keys.search = Some(k.1.to_string())
            },                   

            "sort" => match keys.sort {
                Some(_) => {},
                None => keys.sort = Some(k.1.to_string())
            },                   

            "favourites" => match keys.favourites {
                Some(_) => {},
                None => keys.favourites = Some(k.1.to_string())
            },                   

            _ => {}

        }

    }

    // return the new keybindings
    keys

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
            // replace all None values in the Keys struct with the default ones
            config.keys = change_default_keys(config.keys);
            
            config

        },

        Err(_) => {
            // else use the default config
            println!("Could not read configuration file at '$HOME/.config/rufm/config.ini'! Using default configuration ...");
            Config::default()
        }

    }

}
