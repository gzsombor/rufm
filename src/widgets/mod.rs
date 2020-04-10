/* Principle:
 *
 * Widgets make out the UI.
 * Current widgets:
 *
 *  - Search
 *  - Preview
 *  - FileList
 *  - Favourites
 *
 * All widgets are either Lists, through which you can scroll,
 * or Parapgraphs, which display text (editable or not).
 * Widgets are normally displayed through a struct. These
 * are all stored in the widgets folder and get created by main.rs
 * in the main function. So they have to be publicly imported.
 */

// pub to access it
// from main.rs
pub mod lists;
pub mod paragraphs;
pub mod traits;
pub mod render;

// pub to access it
// from main.rs
pub use lists::files::FileList;
pub use lists::favourites::Favourites;

pub use paragraphs::preview::Preview;
pub use paragraphs::search::Search;
pub use paragraphs::info::Info;

pub use traits::{
    CustomList,
    CustomParagraph,
    EditableParagraph
};

pub use render::draw;

// widgets that can
// be selected -> scrollable or editable
pub enum Selectable {

    Info,
    Search,
    FileList,
    Favourites

}
