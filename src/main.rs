
mod gap_buffer;
mod viewer;
mod text_editor;
use crate::text_editor::TextEditor;
fn main()  {
    let mut text_editor= TextEditor::new();
    text_editor.start_text_editor();
}
