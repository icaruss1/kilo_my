mod editor;
mod terminal;
use editor::Editor;

fn main() {
    // the writer controls the state of the terminal. Not
    // the reader

    Editor::default().run();
}
