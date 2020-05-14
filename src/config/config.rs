// use from other files
use super::*;
use crate::cmd::Options;

// use to read the file
use std::{env::var, fmt::Display, fs::File, io::prelude::Read, process::exit};

impl ConfigOpt {
    fn borders(b: Option<&BordersOpt>) -> Borders {
        // return default borders if nothing was specified
        let borders = match b {
            Some(v) => v,
            None => {
                return Borders {
                    search: [191, 97, 106],
                    info: [208, 135, 112],
                    filelist: [208, 203, 139],
                    preview: [163, 190, 140],
                    favourites: [180, 142, 173],
                }
            }
        };

        // fill in the default values
        Borders {
            search: borders.search.unwrap_or([191, 97, 106]),
            info: borders.info.unwrap_or([208, 135, 112]),
            filelist: borders.filelist.unwrap_or([208, 203, 139]),
            preview: borders.preview.unwrap_or([163, 190, 140]),
            favourites: borders.favourites.unwrap_or([180, 142, 173]),
        }
    }

    fn keys(k: Option<&KeysOpt>) -> Keys {
        // return default keys if nothing was specified
        let keys = match k {
            Some(v) => v,
            None => {
                return Keys {
                    rename: "R".to_string(),
                    copy: "C".to_string(),
                    paste: "P".to_string(),
                    delete: "D".to_string(),
                    search: "/".to_string(),
                    sort: "\t".to_string(),
                    favourites: "F".to_string(),
                    select: " ".to_string(),
                    command: ":".to_string(),
                }
            }
        };

        // fill in the default values
        Keys {
            rename: keys.rename.clone().unwrap_or("R".to_string()),
            copy: keys.copy.clone().unwrap_or("C".to_string()),
            paste: keys.paste.clone().unwrap_or("P".to_string()),
            delete: keys.delete.clone().unwrap_or("D".to_string()),
            search: keys.search.clone().unwrap_or("/".to_string()),
            sort: keys.sort.clone().unwrap_or("\t".to_string()),
            favourites: keys.favourites.clone().unwrap_or("F".to_string()),
            select: keys.select.clone().unwrap_or(" ".to_string()),
            command: keys.command.clone().unwrap_or(":".to_string()),
        }
    }

    fn favourites(f: Option<&FavouritesOpt>) -> Favourites {
        let default = || Favourites {
            names: vec!["Root".to_string(), "Home".to_string()],
            paths: vec!["/".to_string(), "~".to_string()],
        };

        // return default favourites in nothing was specified
        let favourites = match f {
            Some(v) => v,
            None => return default(),
        };

        // check if they have the same length
        // or are of the same option type
        // else report
        let names = favourites.names.clone();
        let paths = favourites.paths.clone();

        if names.is_none() && paths.is_none() {
            return default()
        } else if let Some(n) = names {
            if let Some(p) = paths {
                if n.len() == p.len() {
                    return Favourites { names: n, paths: p };
                }
            }
        }

        // if none of these matched, exits with an error
        report("The arrays names and paths are not of the same length!");
        exit(1);
    }

    fn highlights(h: Option<&HighlightsOpt>) -> Highlights {
        // return default highlights if nothing was specified
        let highlights = match h {
            Some(v) => v,
            None => {
                return Highlights {
                    border: [132, 150, 232],
                    text: Color {
                        fg: Some([132, 150, 232]),
                        bg: None,
                    },
                    symbol: "> ".to_string(),
                }
            }
        };

        // fill in the default values
        Highlights {
            border: highlights.border.clone().unwrap_or([132, 150, 232]),
            text: highlights.text.clone().unwrap_or(Color {
                fg: Some([132, 150, 232]),
                bg: None,
            }),
            symbol: highlights.symbol.clone().unwrap_or("> ".to_string()),
        }
    }

    fn other(o: Option<&OtherOpt>) -> Other {
        // return the default options if nothing was specified
        let other = match o {
            Some(v) => v,
            None => {
                return Other {
                    open_cmd: "$EDITOR".to_string(),
                }
            }
        };

        Other {
            open_cmd: other.open_cmd.clone().unwrap_or("$EDITOR".to_string()),
        }
    }

    // parse all the optional values into
    // new structs with almost no options
    // add default values if None
    fn parse(&self) -> Config {
        Config {
            borders: Self::borders(self.borders.as_ref()),
            highlights: Self::highlights(self.highlights.as_ref()),
            favourites: Self::favourites(self.favourites.as_ref()),
            keys: Self::keys(self.keys.as_ref()),
            other: Self::other(self.other.as_ref()),
        }
    }

    // create a new config struct with default options
    fn default() -> ConfigOpt {
        ConfigOpt {
            borders: None,
            highlights: None,
            favourites: None,
            keys: None,
            other: None,
        }
    }
}

// reports an error in the configuration file
// and ends the process
fn report<T: Display>(error: T) {
    // print a newline at the beginning and at the end
    println!(
"
There is an error in the configuration file:
{}
"
    , error);
}

// reads from the config file at filename and
// parse the value in the optional struct, afterwards
// it parse the optional values to normal values while substituting
// defualts for None values
pub fn create_config(options: &Options) -> Config {
    // read from the file
    let mut content = String::new();
    // get the home directory
    let home = var("HOME").expect("Could not get $HOME!");
    let filename = match &options.config {
        // use the inputted filename
        Some(path) => path.replace("~", home.as_str()),
        // use the default one
        None => format!("{}/.config/rufm/config.ini", home),
    };

    // open the file and check if it exists
    let mut config: Config;
    match File::open(&filename) {
        Ok(v) => {
            // if it exists, assign it
            let mut config_file = v;
            // read it
            if config_file.read_to_string(&mut content).is_err() {
                report(format!("Could not read the config file at {}!", filename));
                exit(1);
            }
            // parse the variable to the Config struct
            let opt: ConfigOpt = match toml::from_str(&content) {
                Ok(v) => v,
                Err(_) => {
                    report("Could not parse toml!");
                    exit(1);
                }
            };
            // remove all the optional values 
            config = opt.parse();
            // replace all ~ with $HOME var
            config.favourites.paths = config
                .favourites
                .paths
                .iter()
                .map(|x| x.replace("~", home.as_str()))
                .collect();
        }
        Err(_) => {
            // use the default config
            config = ConfigOpt::default().parse();
        }
    }
    config
}
