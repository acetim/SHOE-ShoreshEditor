
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use std::io::{self, Write, stdout, Stdout};

use crossterm::cursor;
use crossterm::event::{ KeyCode, KeyEvent};
use crossterm::ExecutableCommand;

pub struct Viewer{
    terminal_size:(u16,u16),
    stdout: Stdout
}
impl Viewer{
    pub fn new() -> Self {
        Self {terminal_size:crossterm::terminal::size().unwrap_or((20,20)), stdout:stdout()}
    }
    pub fn init(&self){
        enable_raw_mode().unwrap();
        self.clear_screen().unwrap();

    }
    pub fn close_viewer(&self){
        self.clear_screen_for_viewer_closure().unwrap();
        disable_raw_mode().unwrap();
    }
    pub fn read_to_viewer(&mut self,key_event: &KeyEvent)->io::Result<()>{
        match key_event.code {
            KeyCode::Char(c) => {
                print!("{}", c);
                self.stdout.flush()?;
                self.stdout.execute(cursor::MoveLeft(2))?;
            }
            KeyCode::Enter=>{
                print!("\n\r");
                self.stdout.execute(cursor::MoveToColumn(self.terminal_size.0))?;
                print!(" ");
                self.stdout.execute(cursor::MoveLeft(1))?;
            }
            KeyCode::Backspace => {
                self.stdout.execute(cursor::MoveRight(1))?;
                print!(" ");
                self.stdout.execute(cursor::MoveLeft(1))?;
            }
            _ => {}
        }
        self.stdout.flush()?;
        Ok(())
 }
    fn clear_screen(&self)->io::Result<()>{
        let cols = self.terminal_size.0;
         for _rows in 0..self.terminal_size.1{
             stdout().execute(cursor::MoveToColumn(cols))?;
             print!("~");
             print!("\n")
         }
        stdout().execute(cursor::MoveToRow(0))?;
        print!(" ");
        stdout().execute(cursor::MoveLeft(1))?;
        Ok(())
    }
    fn clear_screen_for_viewer_closure(&self)-> io::Result<()>{
        stdout().execute(cursor::MoveToColumn(0))?;
        for _rows in 0..self.terminal_size.1*2{
            print!("\n")
        }
        Ok(())
    }

}