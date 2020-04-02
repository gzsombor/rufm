// pub to access it
// from main.rs
pub mod list;
pub mod preview;

extern crate alloc;
use alloc::borrow::Cow;

// pub to access it
// from main.rs
pub use list::FileList;
pub use preview::Preview;

use tui::terminal::Terminal;
use tui::backend::Backend;

use tui::style::{Color, Style};
use tui::layout::{Layout, Direction, Constraint, Alignment};
use tui::widgets::{Text, Paragraph, Block, Borders, List};

// draws the layout
// parameters are a little messed up
pub fn draw_layout<B: Backend> // <Backend: tui::backend::Backend>
    (selected: i8, preview: &mut Preview, 
     text: &Vec<Text>, fl: &FileList, terminal: &mut Terminal<B>) {

    // update the preview
    preview.set_filename(fl.content[fl.current].clone());
    preview.update_content();

    let custom_border_style = Style::default().fg(Color::Red);
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
            )
            .split(term_size);

        let chunks_bottom = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(chunks_top[1]);

        // search paragraph
        let mut search = Paragraph::new(text.iter())
            .block(custom_block.title("Search"))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);
        
        // select the current element
	    let fl_items = fl.content.iter().enumerate().map(|(index, file)| {
		    if index == fl.current {
	            Text::Styled(
	                Cow::Borrowed(file),
		            fl.highlight
		        )
		    } else {
		        Text::Raw(Cow::Borrowed(file))
		    }
	    });

        // create the list
        let mut filelist = List::new(fl_items)
            .block(custom_block.title("Files"));
                
        // preview paragraph
        let c = preview.get_content();
        let mut preview = Paragraph::new(c.iter())
            .block(custom_block.title("Preview"))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);

        match selected {
            0 => search = search.block(custom_block.title("Search").border_style(custom_border_style)),
            1 => filelist = filelist.block(custom_block.title("Files").border_style(custom_border_style)),
            // 2 => preview = preview.block(custom_block.title("Preview").border_style(custom_border_style)),
            _ => {}
        }

        f.render(&mut search, chunks_top[0]);
        f.render(&mut filelist, chunks_bottom[0]);
        f.render(&mut preview, chunks_bottom[1]);

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
