mod list;

use tui::Terminal;
use tui::style::{Color, Style};
use tui::layout::{Layout, Direction, Constraint, Alignment};
use tui::widgets::{Text, Paragraph, Block, Borders, List};

// draws the layout
// parameters are a little messed up
pub fn draw_layout<Backend: tui::backend::Backend>(text: &Vec<tui::widgets::Text<'_>>, terminal: &mut Terminal<Backend>) {

    match terminal.draw(|mut f| {

        // layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(90)
                ].as_ref()
            )
            .split(f.size());

        // search paragraph
        let mut search = Paragraph::new(text.iter())
            .block(Block::default().title("Paragraph")
            .borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);

        f.render(&mut search, chunks[0]);

        // create the file list
        let fs = list::FileList::new();
        // style it
        let style = Style::default().fg(Color::Black);
        let items = fs.content.iter().map(|i| Text::raw(i));

        // create the list
        let mut list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("List"))
            .style(style);
        
        f.render(&mut list, chunks[1]);
        
    }) {

        Ok(()) => {},
        Err(e) => {

            println!("Could not draw to terminal: {}", e);
            return;

        }

    }

}
