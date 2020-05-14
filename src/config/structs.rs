// use to decode the toml
use serde_derive::Deserialize;

// Structs, where the toml
// config will get parse into
#[derive(Deserialize, Clone)]
pub struct ConfigOpt {
    pub keys: Option<KeysOpt>,
    pub borders: Option<BordersOpt>,
    pub highlights: Option<HighlightsOpt>,
    pub favourites: Option<FavouritesOpt>,
    pub other: Option<OtherOpt>,
}

#[derive(Deserialize, Clone)]
pub struct KeysOpt {
    pub rename: Option<String>,
    pub copy: Option<String>,
    pub paste: Option<String>,
    pub delete: Option<String>,
    pub search: Option<String>,
    pub sort: Option<String>,
    pub favourites: Option<String>,
    pub select: Option<String>,
    pub command: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct FavouritesOpt {
    pub names: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
}

#[derive(Deserialize, Clone)]
pub struct BordersOpt {
    pub search: Option<[u8; 3]>,
    pub info: Option<[u8; 3]>,
    pub filelist: Option<[u8; 3]>,
    pub preview: Option<[u8; 3]>,
    pub favourites: Option<[u8; 3]>,
}

#[derive(Deserialize, Clone)]
pub struct HighlightsOpt {
    pub border: Option<[u8; 3]>,
    pub text: Option<Color>,
    pub symbol: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct OtherOpt {
    pub open_cmd: Option<String>,
}

// This is the same for both
// because you can have transparent / no colors
#[derive(Deserialize, Clone)]
pub struct Color {
    pub fg: Option<[u8; 3]>,
    pub bg: Option<[u8; 3]>,
}

// Structs, where the toml
// modified config will get parsed into
// after all Option values have been cleared
#[derive(Deserialize)]
pub struct Config {
    pub keys: Keys,
    pub borders: Borders,
    pub highlights: Highlights,
    pub favourites: Favourites,
    pub other: Other,
}

#[derive(Deserialize, Clone)]
pub struct Keys {
    pub rename: String,
    pub copy: String,
    pub paste: String,
    pub delete: String,
    pub search: String,
    pub sort: String,
    pub favourites: String,
    pub select: String,
    pub command: String,
}

#[derive(Deserialize)]
pub struct Favourites {
    pub names: Vec<String>,
    pub paths: Vec<String>,
}

#[derive(Deserialize)]
pub struct Borders {
    pub search: [u8; 3],
    pub info: [u8; 3],
    pub filelist: [u8; 3],
    pub preview: [u8; 3],
    pub favourites: [u8; 3],
}

#[derive(Deserialize)]
pub struct Highlights {
    pub border: [u8; 3],
    pub text: Color,
    pub symbol: String,
}

#[derive(Deserialize)]
pub struct Other {
    pub open_cmd: String,
}
