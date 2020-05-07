mod action;
mod cmd;
mod config;
mod widgets;

// Write
use std::io::{stdin, stdout};

// gets the cwd
use std::env::current_dir;

// widgets
use widgets::{
    // functions
    draw,

    // traits
    CustomList,
    CustomParagraph,
    EditableParagraph,

    // enums
    InfoMode,
    SearchMode,
    Input,
    Confirm,
    Selectable,
    SortStyles,
};

// config
use config::create_config;

// action
use action::Action;

// cmd
use cmd::Options;

// backend
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use tui::backend::TermionBackend;
use tui::Terminal;

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
    let config = create_config(options.config.clone());

    // function to get the first char
    let get_keybind = |key: String, name: &str| {
        // get the chars
        key.chars().next()
            .expect(format!("Keybinding ({}) not a single character!", name).as_str())
    };

    // keybindings
    let key_rename = get_keybind(config.keys.rename, "rename");
    let key_copy = get_keybind(config.keys.copy, "copy");
    let key_paste = get_keybind(config.keys.paste, "paste");
    let key_delete = get_keybind(config.keys.delete, "delete");
    let key_search = get_keybind(config.keys.search, "search");
    let key_sort = get_keybind(config.keys.sort, "sort");
    let key_favourites = get_keybind(config.keys.favourites, "favourites");
    let key_select = get_keybind(config.keys.select, "select");
    let key_command = get_keybind(config.keys.command, "command");

    // Widgets
    let mut search = widgets::Search::new(config.borders.search);
    let mut filelist = widgets::FileList::new(config.borders.filelist, config.other.open_cmd);
    let mut preview = widgets::Preview::new(config.borders.preview);
    let mut favourites = widgets::Favourites::new(
        config.borders.favourites,
        config.favourites.names.clone(),
        config.favourites.paths.clone(),
    );
    let mut info = widgets::Info::new(config.borders.info);
    // current selected element
    let mut selected = Selectable::FileList;
    // actions
    let mut action = Action::new();

    // creating the terminal
    let stdout = stdout()
        .into_raw_mode()
        .expect("Could not draw to the terminal!");
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Could not draw to the terminal!");
    // hide the cursor
    terminal
        .hide_cursor()
        .expect("Could not draw to the terminal!");

    // get the keyboard input
    let mut events = stdin().events();

    // startup notif, only if enabled
    if config.other.startup_info {
        println!("\nConfiguration: {}", options.config);
        // move the cursor to the next line
        let cursor_pos = terminal.get_cursor().unwrap().1;
        terminal
            .set_cursor(0, cursor_pos)
            .expect("Could not move cursor. Please disable the startup_info!");
        println!(
            "Working directory: {}",
            current_dir().expect("Could not get cwd!").display()
        );
        // move the cursor to the next line
        let cursor_pos = terminal.get_cursor().unwrap().1;
        terminal
            .set_cursor(0, cursor_pos)
            .expect("Could not move cursor. Please disable the startup_info!");
        println!("Press any key to start ...");

        // wait for the event to start the file manager
        let _ = events.next();
    }

    // clear the terminal
    terminal.clear().expect("Could not clear the terminal!");

    // some function I'm gonna use a lot
    // as closures, so they can access all variables
    // ... not yet implemented

    // loop through keyboard inputs
    loop {
        // update the filelist
        filelist.update();
        // update the preview
        preview.update(filelist.get_current_selected());
        // update the info
        info.update(filelist.get_current_selected());

        // draw the layout
        draw(
            &selected,
            &config.highlights,
            &info,
            &preview,
            &favourites,
            &search,
            &filelist,
            &mut terminal,
        );

        // get the next event
        let event = match events.next() {
            Some(v) => match v {
                Ok(e) => e,
                Err(_) => continue,
            },
            None => continue,
        };

        // match events
        // specific to selected item
        match selected {
            Selectable::Search => match search.mode {
                SearchMode::Input(v) => match event {
                    // run the current action
                    Event::Key(Key::Char('\n')) => {
                        match v {
                            Input::Rename => {
                                action.rename(
                                    filelist.get_current_selected(),
                                    search.get_content(),
                                );
                                // update info
                                info.content = action.status.clone();
                                info.mode = InfoMode::Status;
                                // clear the selected elements
                                filelist.selected = Vec::new();
                            }

                            Input::Command => {
                                // run the command
                                action.run_cmd(search.get_content());
                                // update info
                                info.content = action.status.clone();
                                info.mode = InfoMode::Status;
                                // clear the selected elements
                                filelist.selected = Vec::new();
                            }

                            Input::Search => {
                                filelist.scroll_top();
                                // set the key and 
                                filelist.key = search.items();
                            }
                        }
                        // clear the search and select the filelist
                        search.clear();
                        selected = Selectable::FileList;
                    }

                    // add the current selected files name
                    Event::Key(Key::Char('\t')) => {
                        match v {
                            Input::Search => {},
                            _ => search.add(filelist.get_current_selected())
                        }
                    }

                    // add the char to the string
                    Event::Key(Key::Char(c)) => {
                        search.add(c.to_string());
                        if let Input::Search = v {
                            filelist.scroll_top();
                            filelist.key = search.items();
                        }
                    }

                    // exit search mode
                    Event::Key(Key::Esc) => {
                        // clear info
                        search.clear();
                        selected = Selectable::FileList;
                        if let Input::Search = v {
                            filelist.sort_style = SortStyles::Normal;
                        }
                    }

                    // remove last char
                    Event::Key(Key::Backspace) => {
                        search.delete();
                        if let Input::Search = v {
                            filelist.scroll_top();
                            filelist.key = search.items();
                        }
                    }

                    _ => {}
                },

                SearchMode::Confirm(v) => match event {
                    // sort the files -> end the search input
                    Event::Key(Key::Char('y')) => {
                        match v {
                            Confirm::Delete => {
                                // delete the file
                                action.delete(
                                    filelist.selected.clone(),
                                    filelist.get_current_selected(),
                                );
                                // update info
                                info.content = action.status.clone();
                                info.mode = InfoMode::Status;
                                selected = Selectable::FileList;
                                // scroll to the top of the filelist
                                filelist.scroll_top();
                                filelist.selected = Vec::new();
                            }
                        }
                    }

                    // exit search mode
                    Event::Key(Key::Esc) | Event::Key(Key::Char('n')) => {
                        // clear the info
                        search.clear();
                        info.mode = InfoMode::Information;
                        selected = Selectable::FileList;
                    }

                    _ => {}
                },

                _ => {}

            },

            Selectable::FileList => match event {
                // quit
                Event::Key(Key::Char('q')) => {
                    terminal.clear().expect("Could not clear the terminal!");
                    break;
                }

                // activate searching mode
                Event::Key(Key::Char(c)) if c == key_search => {
                    selected = Selectable::Search;
                    // set the sorting style to search
                    filelist.sort_style = SortStyles::Search;
                    search.clear();
                }

                // scroll down
                Event::Key(Key::Char('j')) | Event::Key(Key::Down) => {
                    filelist.scroll_down();
                }

                // scroll up
                Event::Key(Key::Char('k')) | Event::Key(Key::Up) => {
                    filelist.scroll_up();
                }

                // change one dir back
                Event::Key(Key::Char('h')) | Event::Key(Key::Left) => {
                    if let Err(e) = filelist.change_dir_back() {
                        info.content = e;
                        info.mode = InfoMode::Status;
                    } else {
                        filelist.scroll_top();
                    }
                }

                // change one dir back
                Event::Key(Key::Char('l')) | Event::Key(Key::Right) => {
                    if let Err(e) = filelist.change_dir_selected() {
                        info.content = e;
                        info.mode = InfoMode::Status;
                    } else {
                        filelist.scroll_top();
                    }
                }

                // change to favourites
                Event::Key(Key::Char(c)) if c == key_favourites => {
                    selected = Selectable::Favourites;
                }

                // delete the file / directory
                Event::Key(Key::Char(c)) if c == key_delete => {
                    // update the info graph
                    search.content = "Really? (y/n)".to_string();
                    search.mode = SearchMode::Confirm(Confirm::Delete);
                    // change selected field
                    selected = Selectable::Search;
                }

                // copy the file / directory
                Event::Key(Key::Char(c)) if c == key_copy => {
                    action.copy(filelist.selected.clone(), filelist.get_current_selected());
                    filelist.scroll_top();
                    filelist.selected = Vec::new();
                    // update info
                    info.content = action.status.clone();
                    info.mode = InfoMode::Status;
                }

                // paste the file / directory
                Event::Key(Key::Char(c)) if c == key_paste => {
                    action.paste();
                    filelist.scroll_top();
                    // update info
                    info.content = action.status.clone();
                    info.mode = InfoMode::Status;
                }

                // rename the file / directory
                Event::Key(Key::Char(c)) if c == key_rename => {
                    // update info
                    search.clear();
                    // change selected field
                    selected = Selectable::Search;
                    search.mode = SearchMode::Input(Input::Rename);
                }

                // toggle sorting
                Event::Key(Key::Char(c)) if c == key_sort => {
                    // update the sorting style
                    filelist.toggle_sort_style();
                }

                // toggle selecting
                Event::Key(Key::Char(c)) if c == key_select => {
                    // toggle the selecting
                    filelist.toggle_select();
                }

                // get into the command mode
                // allows you to run shell commands
                Event::Key(Key::Char(c)) if c == key_command => {
                    // update info
                    search.clear();
                    // change selected field
                    selected = Selectable::Search;
                    search.mode = SearchMode::Input(Input::Command);
                }

                // open the selected file / directory
                Event::Key(Key::Char('\n')) => {
                    // oepn the file
                    if let Err(e) = filelist.open() {
                        // update the info
                        info.content = e.to_string();
                        info.mode = InfoMode::Status;
                    } else {
                        // when the file
                        // gets closed, hide the cursor
                        // for some reason it appears again
                        terminal.hide_cursor().expect("Could not hide the cursor!");
                    }
                }

                _ => {}
            },

            Selectable::Favourites => match event {
                // quit
                Event::Key(Key::Char('q')) => {
                    terminal.clear().expect("Failed to clear terminal!");
                    break;
                }

                // scroll down
                Event::Key(Key::Char('j')) | Event::Key(Key::Down) => {
                    favourites.scroll_down();
                }

                // scroll up
                Event::Key(Key::Char('k')) | Event::Key(Key::Up) => {
                    favourites.scroll_up();
                }

                // select the favourite
                Event::Key(Key::Char('\n')) => {
                    selected = Selectable::FileList;
                    favourites.change_dir_selected();
                    filelist.scroll_top();
                }

                // exit search mode
                Event::Key(Key::Esc) => {
                    selected = Selectable::FileList;
                }

                _ => {}
            },
        }
    }
}
