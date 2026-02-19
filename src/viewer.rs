
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use std::io::{self, Read, Write, stdout, Stdout};
use crossterm::event;
use crossterm::cursor;
use crossterm::event::{Event, KeyCode};
use crossterm::ExecutableCommand;

pub struct Viewer{

}
impl Viewer{
    pub fn view(){
        Self::init();


        Self::start_viewer().unwrap();
    }
    fn init(){
        enable_raw_mode().unwrap();
    }
    fn start_viewer()-> io::Result<()>{

        let mut stdout = stdout();
        let start_col = crossterm::terminal::size().unwrap_or((20,20)).0;

        stdout.execute(cursor::MoveToColumn(start_col))?;
        stdout.execute(cursor::MoveLeft(1))?;

        Self::handle_until_esc(&mut stdout,start_col)?;
        disable_raw_mode()?;
        Ok(())
    }
    fn handle_until_esc(stdout:&mut Stdout,start_col:u16)->io::Result<()>{
        loop {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                     print!("{}", c);
                     stdout.flush()?;
                     stdout.execute(cursor::MoveLeft(2))?;
                 }
                    KeyCode::Enter=>{
                     print!("\n\r");
                     stdout.execute(cursor::MoveToColumn(start_col))?;
                     stdout.execute(cursor::MoveLeft(1))?;
                 }
                    KeyCode::Backspace => {
                     stdout.execute(cursor::MoveRight(1))?;
                     print!(" ");
                     stdout.execute(cursor::MoveLeft(1))?;
                 }
                    _ => {}
                }
                stdout.flush()?;
            }
        }
        Ok(())
 }
}