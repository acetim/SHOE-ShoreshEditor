use crossterm::event::KeyCode;
use crate::gap_buffer::{GapBuffer, LeftRight};

pub struct Editor{
    buffer:GapBuffer,
    
}
impl Editor{
    pub fn new(path:&str)->Self{
        Self{buffer:GapBuffer::from_file(path,1024)}
    }
    pub fn handle_key(&mut self,key:KeyCode){
        match key {
            KeyCode::Left=>{
                self.buffer.move_left_or_right(LeftRight::Left);
            }
            KeyCode::Right=>{
                self.buffer.move_left_or_right(LeftRight::Right);
            }
            KeyCode::Backspace=>{
                self.buffer.delete();
            }
            KeyCode::Enter=>{
                self.buffer.insert('\n');
            }
            KeyCode::Char(char)=>{
                self.buffer.insert(char);
            }
            KeyCode::Tab=>{
                for _i in 0..4{
                    self.buffer.insert(' ');
                }
            }
            _=>{
                
            }
        }
    }

    pub fn buffer(&self) -> &GapBuffer {
        &self.buffer
    }
}