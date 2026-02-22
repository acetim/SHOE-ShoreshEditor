use std::io;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crate::editor::Editor;
use crate::viewer::Viewer;

pub struct TextEditor{
    viewer:Viewer,
    editor:Editor
}

impl TextEditor{
    pub fn new(path:&str) -> Self {
        Self {viewer:Viewer::new(),editor:Editor::new(path)}
    }
    pub fn start_text_editor(&mut self){
        self.viewer.init();
        self.viewer.render(self.editor.buffer()).unwrap();
        match self.text_editor_loop(){
            Ok(())=>{}
            Err(error)=>{
                eprintln!("{}",error.to_string());
            }
        }
        //TODO save file and handle errors
        self.viewer.close_viewer()
        //TODO close editor
    }
    fn text_editor_loop(&mut self)->io::Result<()>{
        loop{
            if let Event::Key(key_event) = event::read()? {
                match key_event.code{
                    KeyCode::Esc=>{
                        break
                    }
                    KeyCode::Left|KeyCode::Right|KeyCode::Backspace|KeyCode::Enter=>{
                        self.editor.handle_key(key_event.code);
                        self.viewer.render(self.editor.buffer())?;
                    }
                    KeyCode::Char(_c)=>{
                        self.editor.handle_key(key_event.code);
                        self.viewer.render(self.editor.buffer())?;
                    }
                    _=>{}
                }
            }
        }
        Ok(())
    }
}