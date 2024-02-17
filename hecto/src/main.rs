mod editor;
mod terminal;
mod document;
mod row;

pub use document::Document;
pub use terminal::Terminal;
pub use row::Row;
use editor::Editor;
pub use editor::Position;

fn main() {
    Editor::default().run();
}