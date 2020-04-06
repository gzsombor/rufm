/* Principle:
 *
 * Every action implements the action trait.
 * This enables the code to treat all actions
 * the same way, so no code duplication!
 *
 * Actions are structs with the following properties:
 *
 *  - key
 *  - status message
 * 
 * and the following ground methods:
 *
 *  - action method
 */

use crate::widgets::Info;
use termion::event::Key;

// action trait
// needs to be implemented to all structs
pub trait Action {

    fn get_key(&self) -> Key;
    fn status(&self) -> String; 

    fn action(&self);

    // matches key
    // to input and if successful
    // runs the action
    fn match_key(&self, input: Key, info: &Info) {
        let key = self.get_key();
        match input {
            // if key matches
            key =>  {
                // run the action
                self.action();
                // send the status message
                info.content = String::from(self.status());
            },
            _ => {}
        } 
    }

}
