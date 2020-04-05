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

use termion::event::Key;

// action trait
// needs to be implemented to all structs
trait Action {

    fn get_key(&self) -> Key;
    fn status(&self) -> String;
    fn action(&self);

}
