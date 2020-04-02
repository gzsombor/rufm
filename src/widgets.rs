mod list;

extern crate alloc;
use alloc::borrow::Cow;

use tui::terminal::Terminal;
use tui::style::{Color, Style};
use tui::layout::{Layout, Direction, Constraint, Alignment};
use tui::widgets::{Text, Paragraph, Block, Borders, List};

use tui::backend::Backend;

// create and returns a file list
pub fn create_rows() -> list::FileList {

    list::FileList::new()

}

// draws the layout
// parameters are a little messed up
pub fn draw_layout<Backend: tui::backend::Backend>
    (text: &Vec<Text<'_>>, fl: &list::FileList, terminal: &mut Terminal<Backend>) {

    match terminal.draw(|mut f| {

        // layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                    Constraint::Percentage(10),
                    Constraint::Percentage(90)
                ].as_ref()
            )
            .split(f.size());

        // search paragraph
        let mut search_pgraph = Paragraph::new(text.iter())
            .block(Block::default().title("Paragraph")
            .borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);

        f.render(&mut search_pgraph, chunks[0]);

        // style it
        // let style = Style::default().fg(Color::Black);
        // let items = rows.content.iter().map(|i| Text::raw(i));

        // style the current element
        let items = fl.content.iter().enumerate().map(|(index, file)| {
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
        let mut list = List::new(items)
            .block(Block::default()
            .borders(Borders::ALL)
            .title("List"));
        
        f.render(&mut list, chunks[1]);

    }) {
        // is sucessful
        Ok(()) => {},
        // else
        Err(e) => {
            println!("Could not draw to terminal: {}", e);
            return;
        }
    }
}
