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
use widgets::Selectable;
use widgets::CustomList;
use widgets::CustomParagraph;
use widgets::draw;

// config
use config::create_config;

// action
use action::Action;

// cmd
use cmd::eval;

// backend
use termion::raw::IntoRawMode;
use termion::event::{Key, Event};
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::{TermionBackend};


// entry point
fn main() {

    eval();
    rufm();

}

// tui
fn rufm() {

    // creating the terminal
    let stdout = stdout().into_raw_mode().expect("Could not draw to the terminal!");
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Could not draw to the terminal!");
    // hide the cursor
    terminal.hide_cursor().expect("Could not draw to the terminal!");

    // clear the terminal
    terminal.clear().expect("Could not clear the terminal!");

    // get the config
    let config = create_config();

    // Widgets
    let mut search = widgets::Search::new();
    let mut filelist = widgets::FileList::new();
    let mut preview = widgets::Preview::new();
    let mut favourites = widgets::Favourites::new(
        config.favourites.names,
        config.favourites.paths
    );
    let mut info = widgets::Info::new();

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
    draw(&selected, &info, &preview, &favourites, &search, &filelist, &mut terminal);

    // for keyboard input
    let stdin = stdin();

    // loop through keyboard inputs
    // and evaluate them
    for evt in stdin.events() {

        let event = evt.unwrap(); 
        // match events
        // specific to selected item
        match selected {

            Selectable::Search => match event {

                // sort the files -> end the search input
                Event::Key(Key::Char('\n')) => {
                    filelist.scroll_top();
                    filelist.sort(search.items());
                    filelist.update = false;
                    selected = Selectable::FileList;
                },

                // add the char to the string
                Event::Key(Key::Char(c)) => {
                    search.add(c.to_string());
                    filelist.scroll_top();
                    filelist.sort(search.items());
                },

                // exit search mode
                Event::Key(Key::Esc) => {
                    filelist.update = true;
                    search.clear();
                    selected = Selectable::FileList;
                },

                // remove last char
                Event::Key(Key::Backspace) => {
                    search.delete();
                    filelist.scroll_top();
                    filelist.sort(search.items());
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
	            Event::Key(Key::Char('/')) => {
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
                Event::Key(Key::Char('F')) => {
                    selected = Selectable::Favourites;
                },

                // delete the file / directory 
                Event::Key(Key::Char('X')) => {
                    action.delete(filelist.get_current());
                    // update the info graph
                    info.update = false;
                    info.content = action.status.clone();
                },

                // copy the file
                Event::Key(Key::Char('C')) => {
                    action.copy(filelist.get_current());
                    // update info
                    info.update = false;
                    info.content = action.status.clone();
                },

                // paste the file
                Event::Key(Key::Char('P')) =>  {
                    action.paste();
                    // update info
                    info.update = false;
                    info.content = action.status.clone();
                },

	            _ => {}
	
	        },
            
            Selectable::Favourites => match event {
                
                // quit
	            Event::Key(Key::Char('q')) => {
	                terminal.clear().expect("Failed to clear terminal!");
	                break;
	            },

                // activate searching mode
	            Event::Key(Key::Char('/')) => {
	                selected = Selectable::Search;
                    search.clear();
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
        draw(&selected, &info, &preview, &favourites, &search, &filelist, &mut terminal);
        
    }
    
}
