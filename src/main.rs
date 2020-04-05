mod widgets;

extern crate alloc;
use alloc::borrow::Cow;

// Write
use std::io::{stdout, stdin, Error};

use widgets::Selectable;

use widgets::traits::ScrollableList;
use widgets::traits::CustomParagraph;

use widgets::draw;

// backend
use termion::raw::IntoRawMode;
use termion::event::{Key, Event};
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::{TermionBackend};

use tui::widgets::Text;

use std::env::current_dir;


// entry point
fn main() -> Result<(), Error> {
    
    // creating the terminal
    let stdout = stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    match terminal.clear() {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to clear terminal: {}", e);
            return Err(e);
        }
    }
    
    // text for the paragraph
    let mut search = widgets::Search::new();
    // file list
    let mut filelist = widgets::FileList::new();
    // text for preview
    let mut prev = widgets::Preview::new();
    // favourties tab
    let mut favourites = widgets::Favourites::new();
    // info paragraph
    let mut info = widgets::Info::new();
    // current selected element
    let mut selected = Selectable::FileList;

    filelist.update();

    // draw the layout for the first time   
    draw(&selected, &mut info, &mut prev, &favourites, &search, &filelist, &mut terminal);

    // for keyboard input
    let stdin = stdin();

    let mut searching = false;

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
                    selected = Selectable::FileList;
                }

                // add the char to the string
                Event::Key(Key::Char(c)) => search.add(c.to_string()),

                // exit search mode
                Event::Key(Key::Esc) => {
                    selected = Selectable::FileList;
                },

                // remove last char
                Event::Key(Key::Backspace) => search.delete(),

                _ => {}

            },

	        Selectable::FileList => match event {
	            
	            // quit
	            Event::Key(Key::Char('q')) => {
	                match terminal.clear() {
	                    Ok(_) => {},
	                    Err(e) => {
	                        println!("Failed to clear terminal: {}", e);
	                        return Err(e);
	                    }
	                }
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
                }

	            _ => {}
	
	        },
            
            Selectable::Favourites => match event {
                
                // quit
	            Event::Key(Key::Char('q')) => {
	                match terminal.clear() {
	                    Ok(_) => {},
	                    Err(e) => {
	                        println!("Failed to clear terminal: {}", e);
	                        return Err(e);
	                    }
	                }
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
                    filelist.update();
                }

                // exit search mode
                Event::Key(Key::Esc) => {
                    selected = Selectable::FileList;
                },

	            _ => {}

            }

        }

        // draw the layout
        draw(&selected, &mut info, &mut prev, &favourites, &search, &filelist, &mut terminal);
        
    }

    Ok(())
    
}
