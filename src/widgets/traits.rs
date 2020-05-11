/* Principle:
 *
 * These traits have to be implemented on
 * all Lists/Paragraphs.
 */

use std::borrow::Cow;

use tui::style::Style;
use tui::widgets::Text;

// gets implemented on all
// scrollable lists
pub trait CustomList {
    fn get_len(&self) -> usize;
    fn get_items(&self) -> Vec<String>;
    fn get_current(&self) -> usize;
    fn set_current(&mut self, new: usize);

    // returns the elements in a way
    // it can be used with a List
    fn display(&self) -> Vec<Text> {
        // get the elements and
        let elements = self.get_items();
        // convert all strings to a text::raw object
        elements.iter()
            .map(|i| Text::raw(i.clone()))
            .collect::<Vec<Text>>()
    }

    // scrolls up in the list
    fn scroll_up(&mut self) {
        let cur = self.get_current();
        if cur == 0 {
            self.set_current(self.get_len() - 1);
        } else {
            self.set_current(cur - 1);
        }
    }

    // scrolls down in the list
    fn scroll_down(&mut self) {
        let cur = self.get_current();
        if cur == self.get_len() - 1 {
            self.set_current(0);
        } else {
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
}

// gets implemented on all paragraphs
pub trait CustomParagraph {
    // return all important items
    // for displaying
    fn items(&self) -> String;

    fn display(&self) -> Vec<Vec<Text>> {
        // get all important items
        let content: Vec<String> = self.items()
            .split('\t').map(|x| x.to_string()).collect();
        // create a vector out of
        // the input string which can
        // be used with Paragraph::new()
        let mut text = Vec::new();
        for i in content {
            // create a vector with this element
            let v = vec![Text::Styled(Cow::Owned(i), Style::default())];
            // push it to the vector with all texts
            text.push(v);
        }
        // return the texts vector
        text.clone()
    }
}

// gets implemented on all paragraphs
pub trait EditableParagraph {
    // return all important items
    // for displaying
    fn get_content(&self) -> String;
    fn set_content(&mut self, new: String);

    // updates the string
    // with the input char
    fn add(&mut self, new: String) {
        self.set_content(format!("{}{}", self.get_content(), new));
    }

    // pop the last element of the string
    // = Backspace
    fn delete(&mut self) {
        let mut c = self.get_content();
        c.pop();
        self.set_content(c);
    }

    // clear the content
    // get called when new search started
    fn clear(&mut self) {
        self.set_content(String::new());
    }
}
