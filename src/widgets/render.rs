use std::env::current_dir;

use tui::terminal::Terminal;
use tui::backend::Backend;

use tui::style::{Color, Style, Modifier};
use tui::layout::{Layout, Direction, Constraint, Alignment};
use tui::widgets::{Paragraph, Block, Borders, SelectableList};

use super::*;

use crate::config::create_config;

// draws the layout
// parameters are a little messed up
pub fn draw<B: Backend> // <Backend: tui::backend::Backend>
    (selected: &Selectable, info: &Info, preview: &Preview,
    favs: &Favourites, search: &Search, fl: &FileList, terminal: &mut Terminal<B>) {


    // read from the configuration and
    // update specific values
    let config = create_config();
    
    let b_h = config.colors.border_highlight;
    let b_n = config.colors.border_normal;
    let t_h = config.colors.text_highlight;

    // custom colors
    let mut custom_select_style = Style::default()
        .modifier(Modifier::BOLD);

    match t_h.fg {
        Some(v) => { custom_select_style = custom_select_style.fg(Color::Rgb(v[0], v[1], v[2])); },
        None => {}
    }

    match t_h.bg {
        Some(v) => { custom_select_style = custom_select_style.bg(Color::Rgb(v[0], v[1], v[2])); },
        None => {}
    }

    let custom_border_style_normal = Style::default()
        .fg(Color::Rgb(b_n[0], b_n[1], b_n[2]));
    let custom_border_style_selected = Style::default()
        .fg(Color::Rgb(b_h[0], b_h[1], b_h[2]));

    // custom block
    let custom_block = Block::default().borders(Borders::ALL);


    terminal.draw(|mut f| {


        // layout
        
        // splits the screen in a small top row
        // and a big bottom row
        let chunks_vert = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                    Constraint::Length(3),
                    Constraint::Min(20)
                ].as_ref()
            ).split(f.size());

        // splits the top row into a big
        // search bar and a small info bar
        let chunks_top = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([
                    Constraint::Percentage(70),
                    Constraint::Percentage(30)
                ].as_ref()
            ).split(chunks_vert[0]);

        // splits the bottom into a fileslist
        let chunks_bottom = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            ).split(chunks_vert[1]);

        // splits the bottom right half into a big
        // preview and a small favourites list
        let chunks_bottom_right = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                    Constraint::Percentage(70),
                    Constraint::Percentage(30)
                ].as_ref()
            ).split(chunks_bottom[1]);



        // search paragraph
        let search_display = search.display();
        let mut search_pgraph = Paragraph::new(search_display.iter())
            .block(custom_block.title(" Search ").border_style(custom_border_style_normal))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);



        // info paragraph
        let info_display = info.display();
        let mut info_pgraph = Paragraph::new(info_display.iter())
            .block(custom_block.title(" Info ").border_style(custom_border_style_normal))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);

     

        // create the lists
        let cwd = current_dir().expect("Could not get the cwd!");
        let file_list_title = format!(" -> {} ", cwd.display());
        let mut file_list = SelectableList::default()
            .items(&fl.content)
            .block(custom_block.title(file_list_title.as_str()).border_style(custom_border_style_normal))
            .highlight_style(custom_select_style)
            .highlight_symbol(">");



        // preview paragraph
        let preview_display = preview.display();
        let mut preview_pgraph = Paragraph::new(preview_display.iter())
            .block(custom_block.title(" Preview ").border_style(custom_border_style_normal))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);



        // favourites list normal
        let mut favourites_list = SelectableList::default()
            .items(&favs.names)
            .block(custom_block.title(" Favourites ").border_style(custom_border_style_normal))
            .highlight_style(custom_select_style)
            .highlight_symbol(">");



        // color the selected list
        match selected {

            Selectable::Search => {
                // add colored border
                search_pgraph = search_pgraph.block(custom_block.title(" Search ").border_style(custom_border_style_selected));
            },

            Selectable::FileList => {
                // add colored border and select the current item
                file_list = file_list 
                    .block(custom_block.title(file_list_title.as_str()).border_style(custom_border_style_selected))
                    .select(Some(fl.current));
            },

            Selectable::Favourites => {
                // add colored border and select the current item
                favourites_list = favourites_list
                    .block(custom_block.title(" Favourites ").border_style(custom_border_style_selected))
                    .select(Some(favs.current));

            },

        }

        
        // render all elements in their chunk
        f.render(&mut search_pgraph, chunks_top[0]);
        f.render(&mut info_pgraph, chunks_top[1]);
        f.render(&mut file_list, chunks_bottom[0]);
        f.render(&mut preview_pgraph, chunks_bottom_right[0]);
        f.render(&mut favourites_list, chunks_bottom_right[1]);

    }).expect("Could not draw to terminal!");

}
