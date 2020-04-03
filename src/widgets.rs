// pub to access it
// from main.rs
pub mod list;
pub mod preview;
pub mod favourites;

extern crate alloc;
use alloc::borrow::Cow;

// pub to access it
// from main.rs
pub use list::FileList;
pub use preview::Preview;
pub use favourites::Favourites;

use tui::terminal::Terminal;
use tui::backend::Backend;

use tui::style::{Color, Style};
use tui::layout::{Layout, Direction, Constraint, Alignment};
use tui::widgets::{Text, Paragraph, Block, Borders, List};

// widgets that can
// be selected and scrolled
pub enum Selectable {

    Search,
    FileList,
    Favourites

}


// draws the layout
// parameters are a little messed up
pub fn draw_layout<B: Backend> // <Backend: tui::backend::Backend>
    (selected: &Selectable, preview: &mut Preview, favs: &Favourites,
     text: &Vec<Text>, fl: &FileList, terminal: &mut Terminal<B>) {

    // update the preview
    preview.set_filename(fl.content[fl.current].clone());
    preview.update_content();

    let custom_border_style = Style::default().fg(Color::Blue);
    let custom_block = Block::default().borders(Borders::ALL);

    match terminal.draw(|mut f| {

        // size of the whole terminal
        let term_size = f.size();


        // layout
        let chunks_top = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                    Constraint::Percentage(10),
                    Constraint::Percentage(90)
                ].as_ref()
            ).split(term_size);

        let chunks_bottom = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            ).split(chunks_top[1]);

        let chunks_right = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                    Constraint::Percentage(70),
                    Constraint::Percentage(30)
                ].as_ref()
            ).split(chunks_bottom[1]);



        // search paragraph
        let mut search = Paragraph::new(text.iter())
            .block(custom_block.title("Search"))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);


      
        // select the current element
	    let fl_items_colored = Box::new(fl.content.iter().enumerate().map(|(index, file)| {
		    if index == fl.current {
	            Text::Styled(
	                Cow::Borrowed(file),
		            fl.highlight
		        )
		    } else {
		        Text::Raw(Cow::Borrowed(file))
		    }
	    }));

        // map the content without highlighting
        let fl_items_normal = fl.content.iter()
            .map(|x| Text::Raw(Cow::Owned(x.clone())));

        // create the lists
        let mut filelist_normal = List::new(fl_items_normal)
            .block(custom_block.title("Files"));

        // create the lists
        let mut filelist_colored = List::new(fl_items_colored)
            .block(custom_block.title("Files")
            .border_style(custom_border_style));



        // preview paragraph
        let prev_content = preview.get_content();
        let mut preview = Paragraph::new(prev_content.iter())
            .block(custom_block.title("Preview"))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);


        // select the current element
	    let favs_items_colored = favs.names.iter().enumerate().map(|(index, name)| {
		    if index == favs.current {
	            Text::Styled(
	                Cow::Borrowed(name),
		            favs.highlight
		        )
		    } else {
		        Text::Raw(Cow::Borrowed(name))
		    }
	    });

        // map the names without highlighting
        let favs_items_normal = favs.names.iter()
            .map(|x| Text::Raw(Cow::Owned(x.clone())));

        let mut favourites_normal = List::new(favs_items_normal) 
            .block(custom_block.title("Favourites"));
       
        let mut favourites_colored = List::new(favs_items_colored) 
            .block(custom_block.title("Favourites"));

       
        // render all elements in their chunk
        f.render(&mut search, chunks_top[0]);
        f.render(&mut filelist_normal, chunks_bottom[0]);
        f.render(&mut preview, chunks_right[0]);
        f.render(&mut favourites_normal, chunks_right[1]);


        // color the selected list
        match selected {

            Selectable::Search => {

                search = search.block(custom_block.title("Search").border_style(custom_border_style));
                f.render(&mut search, chunks_top[0]);

            },

            Selectable::FileList => {

                f.render(&mut filelist_colored, chunks_bottom[0]);

            },

            Selectable::Favourites => {
       
                f.render(&mut favourites_colored, chunks_right[1]);

            },

            _ => {}
        }

        
    }) {
        // is sucessful
        Ok(()) => {},
        // else, exit
        Err(e) => {
            println!("Could not draw to terminal: {}", e);
            return;
        }
    }
}
