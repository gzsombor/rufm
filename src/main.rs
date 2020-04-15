mod widgets;
mod config;
mod action;
mod cmd;

// Write
use std::io::{
    stdout,
    stdin
};

// widgets
use widgets::{

    // traits
    CustomList,
    CustomParagraph,
    EditableParagraph,

    // enums
    InfoMode,
    Selectable,

    // functions
    draw

};

// config
use config::create_config;

// action
use action::{
    Action,
    InputConfirmAction
};

// cmd
use cmd::Options;

// backend
use termion::raw::IntoRawMode;
use termion::event::{Key, Event};
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::{TermionBackend};


// entry point
fn main() {

    rufm();

}

// tui
fn rufm() {

    // evaluate arguments
    let mut options = Options::new();
    options.eval();
    // create configuration
    let config = create_config(options.config);
    // keybindings
    let key_rename = config.keys.rename.chars().nth(0).expect("Keybinding (rename) not a single letter!");
    let key_copy = config.keys.copy.chars().nth(0).expect("Keybinding (copy) not a single letter!");
    let key_paste = config.keys.paste.chars().nth(0).expect("Keybinding (paste) not a single letter!");
    let key_delete = config.keys.delete.chars().nth(0).expect("Keybinding (delete) not a single letter!");
    let key_search = config.keys.search.chars().nth(0).expect("Keybinding (search) not a single letter!");
    let key_sort = config.keys.sort.chars().nth(0).expect("Keybinding (sort) not a single letter!");
    let key_favourites = config.keys.favourites.chars().nth(0).expect("Keybinding (favourites) not a single letter!");

    // creating the terminal
    let stdout = stdout().into_raw_mode().expect("Could not draw to the terminal!");
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Could not draw to the terminal!");
    // hide the cursor
    terminal.hide_cursor().expect("Could not draw to the terminal!");

    // clear the terminal
    terminal.clear().expect("Could not clear the terminal!");
    
    // Widgets
    let mut search = widgets::Search::new();
    let mut filelist = widgets::FileList::new();
    let mut preview = widgets::Preview::new();
    let mut favourites = widgets::Favourites::new(
        config.favourites.names.clone(),
        config.favourites.paths.clone()
    ); let mut info = widgets::Info::new();

    // current selected element
    let mut selected = Selectable::FileList;

    // actions
    let mut action = Action::new();

    // update the filelist
    filelist.update();
    // update the preview
    preview.update(filelist.get_current());
    // update the info
    info.update(filelist.get_current());

    // draw the layout for the first time   
    draw(&selected, &config, &info, &preview, &favourites, &search, &filelist, &mut terminal);

    // for keyboard input
    let stdin = stdin();
    // loop through keyboard inputs
    // and evaluate them
    for evt in stdin.events() {

        let event = evt.unwrap(); 
        // match events
        // specific to selected item
        match selected {

            Selectable::Info => {
               
                match info.mode {

                    InfoMode::Input => match event {

                        // run the current action
                        Event::Key(Key::Char('\n')) => {

                            match action.current {
                            
                                InputConfirmAction::Rename => {

                                    action.rename(filelist.get_current(), info.content.clone());
                                    // update info
                                    info.content = action.status.clone();
                                    info.mode = InfoMode::Status;
                                    selected = Selectable::FileList;
                                    // deselect action
                                    action.current = InputConfirmAction::Nothing;

                                },

                                _ => {}

                            }

                        },
    
                        // add the char to the string
                        Event::Key(Key::Char(c)) => {
                            info.add(c.to_string());
                        },
    
                        // exit search mode
                        Event::Key(Key::Esc) => {

                            // clear info
                            info.clear();
                            info.mode = InfoMode::Information;
                            selected = Selectable::FileList;
                            // deselct action
                            action.current = InputConfirmAction::Nothing;

                        },
    
                        // remove last char
                        Event::Key(Key::Backspace) => {
                            info.delete();
                        },
    
                        _ => {}
                    
                    },

                    InfoMode::Confirmation => match event {

                        // sort the files -> end the search input
                        Event::Key(Key::Char('y')) => {

                            match action.current {

                                InputConfirmAction::Delete => {

                                    // delete the file
                                    action.delete(filelist.get_current());
                                    // update info
                                    info.content = action.status.clone();
                                    info.mode = InfoMode::Status;
                                    selected = Selectable::FileList;
                                    // scroll to the top of the filelist
                                    filelist.scroll_top();
                                    // deselect action
                                    action.current = InputConfirmAction::Nothing;

                                },

                                _ => {}

                            }

                        },
    
                        // exit search mode
                        Event::Key(Key::Esc) | Event::Key(Key::Char('n')) => {

                            // clear the info
                            info.clear();
                            info.mode = InfoMode::Information;
                            selected = Selectable::FileList;
                            // deselect action
                            action.current = InputConfirmAction::Nothing;

                        },
    
                        _ => {}

                    }

                    _ => {}

                }

            },

            Selectable::Search => match event {

                // sort the files -> end the search input
                Event::Key(Key::Char('\n')) => {
                    filelist.scroll_top();
                    // set the key and sort style
                    filelist.key = search.items();
                    filelist.sort_style = 2;
                    selected = Selectable::FileList;
                },

                // add the char to the string
                Event::Key(Key::Char(c)) => {
                    search.add(c.to_string());
                    filelist.key = search.items();
                    filelist.sort_style = 2;
                    filelist.scroll_top();
                },

                // exit search mode
                Event::Key(Key::Esc) => {
                    search.clear();
                    selected = Selectable::FileList;
                },

                // remove last char
                Event::Key(Key::Backspace) => {
                    search.delete();
                    filelist.scroll_top();
                    filelist.key = search.items();
                    filelist.sort_style = 2;
                },

                _ => {}

            },

	        Selectable::FileList => match event {
	            
	            // quit
	            Event::Key(Key::Char('q')) => {
	                terminal.clear().expect("Could not clear the terminal!");
	                break;
	            },
	
	            // activate searching mode
	            Event::Key(Key::Char(c)) if c == key_search => {
	                selected = Selectable::Search;
                    search.clear();
	            },
	
	            // scroll down
	            Event::Key(Key::Char('j')) | Event::Key(Key::Down) => {
	                filelist.scroll_down();
	            },
	            
	            // scroll up
	            Event::Key(Key::Char('k')) | Event::Key(Key::Up) => {
	                filelist.scroll_up();
	            },
	
	            // change one dir back
	            Event::Key(Key::Char('h')) | Event::Key(Key::Left) => {
	                filelist.change_dir_back();
	                filelist.scroll_top();
	            },
	
	            // change one dir back
	            Event::Key(Key::Char('l')) | Event::Key(Key::Right) => {
	                filelist.change_dir_selected();
	                filelist.scroll_top();
	            },
	
                // change to favourites
                Event::Key(Key::Char(c)) if c == key_favourites => {
                    selected = Selectable::Favourites;
                },

                // delete the file / directory 
                Event::Key(Key::Char(c)) if c == key_delete => {
                    // update the info graph
                    info.content = "Really? (y/n)".to_string();
                    info.mode = InfoMode::Confirmation;
                    // change selected field
                    selected = Selectable::Info;
                    // select action
                    action.current = InputConfirmAction::Delete;
                },

                // copy the file / directory
                Event::Key(Key::Char(c)) if c == key_copy => {
                    action.copy(filelist.get_current());
                    filelist.scroll_top();
                    // update info
                    info.content = action.status.clone();
                    info.mode = InfoMode::Status;
                },

                // paste the file / directory
                Event::Key(Key::Char(c)) if c == key_paste =>  {
                    action.paste();
                    filelist.scroll_top();
                    // update info
                    info.content = action.status.clone();
                    info.mode = InfoMode::Status;
                },

                // rename the file / directory
                Event::Key(Key::Char(c)) if c == key_rename => {
                    // update info
                    info.clear();
                    // change selected field
                    selected = Selectable::Info;
                    info.mode = InfoMode::Input;
                    // select action
                    action.current = InputConfirmAction::Rename;
                },

                // toggle sorting
                Event::Key(Key::Char(c)) if c == key_sort => {
                    // update the sorting style
                    filelist.toggle_sort_style();
                },

	            _ => {}
	
	        },
            
            Selectable::Favourites => match event {
                
                // quit
	            Event::Key(Key::Char('q')) => {
	                terminal.clear().expect("Failed to clear terminal!");
	                break;
	            },

	            // scroll down
	            Event::Key(Key::Char('j')) | Event::Key(Key::Down) => {
	                favourites.scroll_down();
	            },
	            
	            // scroll up
	            Event::Key(Key::Char('k')) | Event::Key(Key::Up) => {
	                favourites.scroll_up();
	            },

                // select the favourite
                Event::Key(Key::Char('\n')) => {
                    selected = Selectable::FileList;
                    favourites.change_dir_selected(); 
                    filelist.scroll_top();
                },

                // exit search mode
                Event::Key(Key::Esc) => {
                    selected = Selectable::FileList;
                },

	            _ => {}

            }

        }
       
        // update the filelist
        filelist.update();
        // update the preview
        preview.update(filelist.get_current());
        // update the info
        info.update(filelist.get_current());

        // draw the layout
        draw(&selected, &config, &info, &preview, &favourites, &search, &filelist, &mut terminal);
        
    }
    
}
