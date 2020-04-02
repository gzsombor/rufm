mod widgets;

extern crate alloc;

// Write
use std::io::{stdout, stdin, Error};

use alloc::borrow::Cow;

// backend
use termion::raw::IntoRawMode;
use termion::event::{Key, Event};
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::{TermionBackend};

use tui::widgets::Text;

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
    let mut text = vec![Text::raw(Cow::Owned(String::new()))]; 

    // file list
    let mut filelist = widgets::create_rows();

    // give the drawing
    // simple name function a
    let dl = widgets::draw_layout;
    dl(&text, &filelist, &mut terminal);

    // for keyboard input
    let stdin = stdin();

    let mut searching = false;

    // loop through keyboard inputs
    // and evaluate them
    for evt in stdin.events() {

        let event = evt.unwrap();

        // searching mode 
        // -> update paragraph text
        if searching {

            match event {

                // sort the files -> end the search input
                Event::Key(Key::Char('\n')) => {
                    let search_string = match &text[0] {
                        Text::Raw(Cow::Owned(w)) => w.clone(),
                        _ => "".to_string() 
                    }; filelist.sort(search_string);
                 

                    dl(&text, &filelist, &mut terminal);
                }

                // update string
                Event::Key(Key::Char(c)) => match &mut text[0] {
                    Text::Raw(Cow::Owned(w)) => {
                        // update the text
                        w.push_str(&c.to_string());
                        text[0] = Text::Raw(Cow::Owned(w.clone()));
                    },
                    _ => {}
                },

                // exit search mode
                Event::Key(Key::Esc) => searching = false,

                // remove last char
                Event::Key(Key::Backspace) => {},

                _ => {}

            };

            // update screen
            dl(&text, &filelist, &mut terminal);
            continue;

        } 

        // parse
        // event to function
        match event {
            
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
                searching = true;
                text[0] = Text::Raw(Cow::Owned("".to_string()));
            },

            // scroll down
            Event::Key(Key::Char('j')) | Event::Key(Key::Down) => {
                filelist.scroll_down();
                dl(&text, &filelist, &mut terminal);
            },
            
            // scroll up
            Event::Key(Key::Char('k')) | Event::Key(Key::Up) => {
                filelist.scroll_up();
                dl(&text, &filelist, &mut terminal);
            },

            // change one dir back
            Event::Key(Key::Char('h')) | Event::Key(Key::Left) => {
                filelist.change_dir_back();
                filelist.scroll_top();

                dl(&text, &filelist, &mut terminal);
            },

            // change one dir back
            Event::Key(Key::Char('l')) | Event::Key(Key::Right) => {
                filelist.change_dir_selected();
                filelist.scroll_top();

                dl(&text, &filelist, &mut terminal);
            },

            _ => {}

        };
        
    }

    Ok(())
    
}
