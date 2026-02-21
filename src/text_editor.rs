use std::io;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crate::viewer::Viewer;

pub struct TextEditor{
    viewer:Viewer,

}

impl TextEditor{
    pub fn new() -> Self {
        Self {viewer:Viewer::new()}
    }
    pub fn start_text_editor(&mut self){
        self.viewer.init();
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
                    _=>{
                        //ACTUAL TEXT EDITOR LOOP STARTS HERE
                        
                        //read to buffer
                        //render

                        //TEXT EDITOR LOOP ENDS HERE
                    }
                }
            }
        }
        Ok(())
    }
}