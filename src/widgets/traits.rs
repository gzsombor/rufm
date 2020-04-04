/* Principle:
 *
 * These traits have to be implemented on
 * all Lists/Paragraphs.
 */

extern crate alloc;
use alloc::borrow::Cow;

use tui::style::Style;
use tui::widgets::Text;

// gets implemented on all
// scrollable lists
pub trait ScrollableList {

    fn get_len(&self) -> usize;
    fn get_current(&self) -> usize;
    fn set_current(&mut self, new: usize);

    // scrolls up in the list
    fn scroll_up(&mut self) {

        let cur = self.get_current();
        if cur != 0 {
            self.set_current(cur - 1);
        }

    }

    // scrolls down in the list
    fn scroll_down(&mut self) {

        let cur = self.get_current();
        if cur != self.get_len() - 1 {
            self.set_current(cur + 1);
        }

    }

    // scrolls to the top of the list
    fn scroll_top(&mut self) {
        self.set_current(0);
    }

    // scrolls to the top of the list
    fn scroll_bottom(&mut self) {
        self.set_current(self.get_len());
    }

    fn items(&self) -> (Vec<String>, usize, Style); 

    fn create_colored(&self) -> Vec<Text> { 
       
        let items = self.items();

        let content = items.0;
        let current = items.1;
        let style = items.2;

        // select the current element
        // using .clone() because index and name are references to content
        // so colored can't be returned -> dangling pointers
	    let colored: Vec<Text> = content.iter().enumerate().map(|(index, name)| {
		    if index.clone() == current {
	            Text::Styled(
	                Cow::Owned(name.clone()),
		            style 
		        )
		    } else {
		        Text::Raw(Cow::Owned(name.clone()))
		    }
	    }).collect();

        colored

    }

    fn create_normal(&self) -> Vec<Text> {

        let items = self.items();
        let content = items.0;

        // map the content without highlighting
        let normal: Vec<Text> = content.iter()
            .map(|x| Text::Raw(Cow::Owned(x.clone())))
            .collect();

        normal

    }

}

pub trait CustomParagraph {}
