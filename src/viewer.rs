
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use std::io::{self, Write, stdout, Stdout};

use crossterm::{cursor, terminal};
use crossterm::cursor::SetCursorStyle;
use crossterm::ExecutableCommand;
use crate::gap_buffer::GapBuffer;

pub struct Viewer{
    terminal_size:(u16,u16),
    stdout: Stdout
}
impl Viewer{
    pub fn new() -> Self {
        Self {terminal_size:crossterm::terminal::size().unwrap_or((20,20)), stdout:stdout()}
    }
    pub fn init(&mut self){
        enable_raw_mode().unwrap();
        self.clear_screen().unwrap();
        self.stdout.execute(cursor::Show).unwrap();
        self.stdout.execute(SetCursorStyle::BlinkingBlock).unwrap();
    }
    pub fn close_viewer(&mut self){
        self.clear_screen_for_viewer_closure().unwrap();
        self.stdout.execute(SetCursorStyle::DefaultUserShape).unwrap();
        disable_raw_mode().unwrap();

    }
    pub fn render(&mut self,buffer:&GapBuffer)->io::Result<()>{
        self.stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        self.stdout.execute(cursor::MoveTo(0, 0))?;
        let width = self.terminal_size.0;
        let height= self.terminal_size.1;

        let mut last_line=0;
        let text = buffer.to_string();

        //render text
        for (y, line) in text.lines().enumerate() {

            let line: String = line.chars().rev().collect();

            let line_width = line.chars().count() as u16;
            let start_x = width.saturating_sub(line_width);

            self.stdout.execute(cursor::MoveTo(start_x, y as u16))?;
            print!("{}", line);
            last_line=y;
        }
        //print ~ until the end
        for line in last_line as u16..height{
            self.stdout.execute(cursor::MoveTo(width,line))?;
            print!("~");
            print!("\n");
        }
        //move the cursor to insertion point
        let (x_cursor,y_cursor)=buffer.calculate_cursor_pos();
        let visual_x = width.saturating_sub(1).saturating_sub(x_cursor);
        self.stdout.execute(cursor::MoveTo(visual_x, y_cursor))?;
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