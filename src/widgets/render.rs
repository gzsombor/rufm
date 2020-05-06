use std::env::current_dir;

use tui::backend::Backend;
use tui::terminal::Terminal;

use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListState, Paragraph, Text};

use super::*;

use crate::config::structs::Highlights;

// draws the layout
// parameters are a little messed up
pub fn draw<B: Backend>(
    selected: &Selectable,
    highlights: &Highlights,
    info: &Info,
    preview: &Preview,
    favs: &Favourites,
    search: &Search,
    filelist: &FileList,
    terminal: &mut Terminal<B>,
) {
    // highlighting color and symbol
    let custom_highlight_symbol = highlights.symbol.clone();
    let mut custom_select_style = Style::default().modifier(Modifier::BOLD);

    // add .fg colors
    if let Some(v) = highlights.text.fg {
        custom_select_style = custom_select_style.fg(Color::Rgb(v[0], v[1], v[2]));
    }
    if let Some(v) = highlights.text.bg {
        custom_select_style = custom_select_style.bg(Color::Rgb(v[0], v[1], v[2]));
    }

    // the border highlighting style
    let b = &highlights.border;
    let custom_border_style_selected = Style::default().fg(Color::Rgb(b[0], b[1], b[2]));

    // custom block
    let custom_block = Block::default().borders(Borders::ALL);

    terminal
        .draw(|mut f| {
            // layout

            // splits the screen in a small top row
            // and a big bottom row
            let chunks_vert = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Length(3), Constraint::Min(20)].as_ref())
                .split(f.size());

            // splits the top row into a big
            // search bar and a small info bar
            let chunks_top = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(chunks_vert[0]);

            // splits the bottom into a fileslist
            let chunks_bottom = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks_vert[1]);

            // splits the bottom right half into a big
            // preview and a small favourites list
            let chunks_bottom_right = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(chunks_bottom[1]);

            // search paragraph
            let search_display = search.display();
            let mut search_pgraph = Paragraph::new(search_display.iter())
                .block(
                    custom_block
                        .title(" Search ")
                        .border_style(search.border_style),
                )
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left)
                .wrap(true);

            // info paragraph
            let info_display = info.display();
            let mut info_pgraph = Paragraph::new(info_display.iter())
                .block(
                    custom_block
                        .title(info.get_title())
                        .border_style(info.border_style),
                )
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left)
                .wrap(true);

            // create the lists
            let cwd = current_dir().expect("Could not get the cwd!");
            let file_list_title = format!(" -> {} ", cwd.display());
            let file_list_items = &filelist.display();
            let mut file_list_state = ListState::default();
            let mut file_list = List::new(file_list_items.iter().map(|i| Text::raw(i.clone())))
                .block(
                    custom_block
                        .title(file_list_title.as_str())
                        .border_style(filelist.border_style),
                )
                .highlight_style(custom_select_style)
                .highlight_symbol(custom_highlight_symbol.as_str());

            // preview paragraph
            let preview_display = preview.display();
            let preview_pgraph = Paragraph::new(preview_display.iter())
                .block(
                    custom_block
                        .title(" Preview ")
                        .border_style(preview.border_style),
                )
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left)
                .wrap(true);

            // favourites list normal
            // style the selected items
            let fav_names = favs.names.iter();
            let mut favourites_list = List::new(fav_names.map(|i| Text::raw(i.clone())))
                .block(
                    custom_block
                        .title(" Favourites ")
                        .border_style(favs.border_style),
                )
                .highlight_style(custom_select_style)
                .highlight_symbol(custom_highlight_symbol.as_str());
            let mut favourites_list_state = ListState::default();

            // color the selected list
            match selected {
                Selectable::Info => {
                    // add colored border
                    info_pgraph = info_pgraph.block(
                        custom_block
                            .title(info.get_title())
                            .border_style(custom_border_style_selected),
                    );
                }

                Selectable::Search => {
                    // add colored border
                    search_pgraph = search_pgraph.block(
                        custom_block
                            .title(" Search ")
                            .border_style(custom_border_style_selected),
                    );
                }

                Selectable::FileList => {
                    // add colored border and select the current item
                    file_list = file_list.block(
                        custom_block
                            .title(file_list_title.as_str())
                            .border_style(custom_border_style_selected),
                    );
                    file_list_state.select(Some(filelist.current));
                }

                Selectable::Favourites => {
                    // add colored border and select the current item
                    favourites_list = favourites_list.block(
                        custom_block
                            .title(" Favourites ")
                            .border_style(custom_border_style_selected),
                    );
                    favourites_list_state.select(Some(favs.current));
                }
            }

            // render all elements in their chunk
            f.render_widget(search_pgraph, chunks_top[0]);
            f.render_widget(info_pgraph, chunks_top[1]);
            f.render_stateful_widget(file_list, chunks_bottom[0], &mut file_list_state);

            f.render_widget(preview_pgraph, chunks_bottom_right[0]);
            f.render_stateful_widget(favourites_list, chunks_bottom_right[1], &mut favourites_list_state);
        })
        .expect("Could not draw to terminal!");
}
