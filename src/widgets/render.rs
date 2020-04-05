use tui::terminal::Terminal;
use tui::backend::Backend;

use tui::style::{Color, Style};
use tui::layout::{Layout, Direction, Constraint, Alignment};
use tui::widgets::{Paragraph, Block, Borders, List};

use super::*;

// draws the layout
// parameters are a little messed up
pub fn draw<B: Backend> // <Backend: tui::backend::Backend>
    (selected: &Selectable, info: &mut Info, preview: &mut Preview,
    favs: &Favourites, search: &Search, fl: &FileList, terminal: &mut Terminal<B>) {

    // update the preview
    preview.set_filename(fl.content[fl.current].clone());
    preview.update_content();

    // update the info
    info.update(fl.content[fl.current].clone());

    let custom_border_style = Style::default().fg(Color::Blue);
    let custom_block = Block::default().borders(Borders::ALL);

    match terminal.draw(|mut f| {


        // layout
        let chunks_vert = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                    Constraint::Length(3),
                    Constraint::Min(50)
                ].as_ref()
            ).split(f.size());

        let chunks_top = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([
                    Constraint::Percentage(70),
                    Constraint::Percentage(30)
                ].as_ref()
            ).split(chunks_vert[0]);

        let chunks_bottom = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            ).split(chunks_vert[1]);

        let chunks_bottom_right = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                    Constraint::Percentage(70),
                    Constraint::Percentage(30)
                ].as_ref()
            ).split(chunks_bottom[1]);



        let search_display = search.display_normal();
        // search paragraph
        let mut search_pgraph = Paragraph::new(search_display.iter())
            .block(custom_block.title("Search"))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);


        // update the info
        // info.update();
        let info_display = info.display_normal();
        // info paragraph
        let mut info_pgraph = Paragraph::new(info_display.iter())
            .block(custom_block.title("Info"))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);

     

        // create the lists
        let mut filelist_normal = List::new(fl.create_normal().into_iter())
            .block(custom_block.title("Files"));

        // create the lists
        let mut filelist_colored = List::new(fl.create_colored().into_iter())
            .block(custom_block.title("Files")
            .border_style(custom_border_style));



        let preview_display = preview.display_normal();

        // preview paragraph 
        let mut preview_pgraph = Paragraph::new(preview_display.iter())
            .block(custom_block.title("Preview"))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);



        let mut favourites_normal = List::new(favs.create_normal().into_iter()) 
            .block(custom_block.title("Favourites"));
     
        let mut favourites_colored = List::new(favs.create_colored().into_iter())
            .block(custom_block.title("Favourites"));



        // render all elements in their chunk
        f.render(&mut search_pgraph, chunks_top[0]);
        f.render(&mut info_pgraph, chunks_top[1]);
        f.render(&mut filelist_normal, chunks_bottom[0]);
        f.render(&mut preview_pgraph, chunks_bottom_right[0]);
        f.render(&mut favourites_normal, chunks_bottom_right[1]);


        // color the selected list
        match selected {

            Selectable::Search => {
                search_pgraph = search_pgraph.block(custom_block.title("Search").border_style(custom_border_style));
                f.render(&mut search_pgraph, chunks_top[0]);
            },

            Selectable::FileList => {
                f.render(&mut filelist_colored, chunks_bottom[0]);
            },

            Selectable::Favourites => {
                f.render(&mut favourites_colored, chunks_bottom_right[1]);
            },

        }

        
    }) {
        // is sucessful
        Ok(()) => {},
        // else, exit
        Err(e) => {
            panic!("Could not draw to terminal: {}", e);
        }
    }
}
