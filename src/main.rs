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

    // give the drawing
    // simple name function a
    let dl = widgets::draw_layout;
    dl(&text, &mut terminal);

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

                // update string
                Event::Key(Key::Char(c)) => match &mut text[0] {

                    Text::Raw(Cow::Owned(w)) => {

                        w.push_str(&c.to_string());
                        text[0] = Text::Raw(Cow::Owned(w.clone()));

                    },
                    _ => {}

                },

                // exit search mode
                Event::Key(Key::Esc) => searching = false,

                _ => {}

            };

            // update screen
            widgets::draw_layout(&text, &mut terminal);
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

            // search
            Event::Key(Key::Char('/')) => {
             
                searching = true;
            
            },

            _ => {}

        };

        // stdout().flush().unwrap();

    }

    Ok(())
    
}
