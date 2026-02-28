use std::fs::File;
use std::io;
use std::io::{stdout, Write};
use crossterm::{cursor, event, ExecutableCommand};
use crossterm::event::{Event, KeyCode};
use crate::editor::Editor;
use crate::viewer::Viewer;

pub struct TextEditor{
    viewer:Viewer,
    editor:Editor,
    path:String,
}
pub enum ExitStatus{
    Save,
    Exit,
}

impl TextEditor{
    pub fn new(path:&str) -> Self {
        Self {viewer:Viewer::new(),editor:Editor::new(path),path:path.to_string()}
    }
    pub fn start_text_editor(&mut self)->io::Result<()>{
        loop {
            self.viewer.init();
            self.viewer.init_render(self.editor.buffer())?;
            match self.text_editor_loop() {
                Ok(()) => {}
                Err(error) => {
                    eprintln!("{}", error.to_string());
                }
            }
            match Self::try_exit(){
                ExitStatus::Save=>{
                    let mut file = File::create(&self.path).expect("cannot write to file");
                    file.write_all(self.editor.buffer().to_string().as_bytes())?;
                }
                ExitStatus::Exit=>{
                    break
                }
            }
        }
        self.viewer.close_viewer();
        Ok(())

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
    fn try_exit()->ExitStatus{
        stdout().execute(cursor::MoveTo(0, 0)).unwrap();
        print!("save or exit?(s/e)");
        stdout().flush().expect("error while opening stdout");
        loop {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code{
                    KeyCode::Char(c)=>{
                        match c{
                            's'=>{
                                return ExitStatus::Save
                            }
                            'e'=>{
                                return ExitStatus::Exit
                            }
                            _=>{}
                        }
                    }
                    _=>{}
                }
            }
        }
    }
}