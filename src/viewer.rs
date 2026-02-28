use crossterm::terminal::{enable_raw_mode, ClearType};
use crossterm::terminal::disable_raw_mode;
use std::io::{self, Write, stdout, Stdout};

use crossterm::{cursor, terminal,execute,style::{Stylize, Print},};
use crossterm::cursor::SetCursorStyle;
use crossterm::ExecutableCommand;
use crate::color_tokenizer::ColorTokenizer;
use crate::gap_buffer::GapBuffer;


//TODO OPTIMIZE THE SHIT OUT OF THIS!!!!!!




pub struct Viewer{
    terminal_size:(u16,u16),
    stdout: Stdout,
    colorizer:ColorTokenizer
}
impl Viewer{
    pub fn new() -> Self {
        Self {terminal_size:crossterm::terminal::size().unwrap_or((20,20)), stdout:stdout(),colorizer:ColorTokenizer::new()}
    }
    pub fn init(&mut self){
        enable_raw_mode().unwrap();
        self.stdout.execute(cursor::Show).unwrap();
        self.stdout.execute(SetCursorStyle::BlinkingBlock).unwrap();
    }
    pub fn close_viewer(&mut self){
        self.clear_screen_for_viewer_closure().unwrap();
        self.stdout.execute(SetCursorStyle::DefaultUserShape).unwrap();
        disable_raw_mode().unwrap();

    }
    pub fn render(&mut self,buffer:&GapBuffer)->io::Result<()>{

        let width = self.terminal_size.0;

        let text = buffer.to_string();
        let (x_cursor,y_cursor)=buffer.calculate_cursor_pos();
        //clear current line
        self.stdout.execute(cursor::MoveToRow(y_cursor))?;
        self.stdout.execute(terminal::Clear(ClearType::CurrentLine))?;
        //render text
        for (y, line) in text.lines().skip(y_cursor as usize).enumerate() {

            let line: String = line.chars().rev().collect();

            let line_width = line.chars().count() as u16;
            let start_x = width.saturating_sub(line_width);

            self.stdout.execute(cursor::MoveTo(start_x, (y+(y_cursor as usize)) as u16))?;
            let flipped_line = Self::flip_string(line.as_str());
            let vec_str:Vec<char> = flipped_line.chars().collect();
            execute!(self.stdout,Print(format!("{}",self.colorizer.colorize(&vec_str))))?;//print line
            self.colorizer.reset();
        }

        //move the cursor to insertion point
        let visual_x = width.saturating_sub(x_cursor);
        self.stdout.execute(cursor::MoveTo(visual_x, y_cursor))?;
        //self.stdout.flush()?;

        Ok(())
    }
    pub fn init_render(&mut self,buffer:&GapBuffer)->io::Result<()>{
        self.stdout.execute(cursor::MoveTo(0, 0))?;
        self.stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;


        let width = self.terminal_size.0;

        let text = buffer.to_string();

        //render text

        for (y, line) in text.lines().enumerate() {

            let line: String = line.chars().rev().collect();

            let line_width = line.chars().count() as u16;
            let start_x = width.saturating_sub(line_width);

            self.stdout.execute(cursor::MoveTo(start_x, y as u16))?;
            let flipped_line = Self::flip_string(line.as_str());
            let vec_str:Vec<char> = flipped_line.chars().collect();
            execute!(self.stdout,Print(format!("{}",self.colorizer.colorize(&vec_str))))?;//print line
            self.colorizer.reset();

        }
        //move the cursor to insertion point
        let (x_cursor,y_cursor)=buffer.calculate_cursor_pos();
        let visual_x = width.saturating_sub(x_cursor);

        //init visuals
        for i in y_cursor+1..self.terminal_size.1{
            self.stdout.execute(cursor::MoveTo(self.terminal_size.0-1, i))?;
            print!("~");

        }


        self.stdout.execute(cursor::MoveTo(0, self.terminal_size.1))?;
        execute!(self.stdout,Print(" ".repeat(self.terminal_size.0 as usize).on_cyan()))?;
        self.stdout.execute(cursor::MoveTo(self.terminal_size.0/2-25, self.terminal_size.1))?;
        execute!(self.stdout,Print("SHOE-ShoreshEditor: a shitty text editor by acetim".black().on_cyan()))?;



        self.stdout.execute(cursor::MoveTo(visual_x, y_cursor))?;
        self.stdout.flush()?;

        Ok(())
    }
    fn flip_char(c: char) -> char {
        match c {
            '(' => ')',
            ')' => '(',
            '[' => ']',
            ']' => '[',
            '{' => '}',
            '}' => '{',
            '<' => '>',
            '>' => '<',
            // Add others as needed
            _ => c,
        }
    }
    fn flip_string(input: &str) -> String {
        input.chars().map(Self::flip_char).collect()
    }
    fn clear_screen_for_viewer_closure(&self)-> io::Result<()>{
        stdout().execute(cursor::MoveToColumn(0))?;
        for _rows in 0..self.terminal_size.1*2{
            print!("\n")
        }
        Ok(())
    }



}