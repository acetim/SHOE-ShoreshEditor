use hebrew_unicode_script::is_hbr_consonant;
use std::collections::HashMap;
use colored::Colorize;
use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORDS: HashMap<String, Color> = {
        let mut keywords:HashMap<String,Color>= HashMap::new();
           keywords.insert("שלם".chars().rev().collect::<String>(), Color::Magenta);
           keywords.insert("ויהי_חושך".chars().rev().collect::<String>(), Color::Green);
           keywords.insert("אם_יהיה".chars().rev().collect::<String>(), Color::Cyan);
           keywords.insert("בעוד".chars().rev().collect::<String>(), Color::Cyan);
           keywords.insert("ויאמר".chars().rev().collect::<String>(), Color::Green);
           keywords.insert("ויאמר_שלם".chars().rev().collect::<String>(), Color::Green);
           keywords.insert("ויעש".chars().rev().collect::<String>(), Color::Yellow);
           keywords.insert("ויתם".chars().rev().collect::<String>(),Color::Yellow);
           keywords.insert("ויברא".chars().rev().collect::<String>(), Color::Green);
           keywords.insert("תהו_ובהו".chars().rev().collect::<String>(), Color::Magenta);
           keywords.insert("ויקרא".chars().rev().collect::<String>(),Color::Orange);
           keywords.insert("אמן".chars().rev().collect::<String>(),Color::Green);
           keywords.insert("בסד".chars().rev().collect::<String>(),Color::Blue);
           keywords
    };
}

pub struct ColorTokenizer{
    current:usize

}
pub enum Color{
    Green,
    Blue,
    Magenta,
    Cyan,
    Yellow,
    Orange
}
impl ColorTokenizer{
    pub fn new() -> ColorTokenizer{
        Self{current:0}
    }
    pub fn reset(&mut self){
        self.current = 0;
    }
    pub fn colorize(&mut self,line:&Vec<char>)->String{
        let mut new = String::new();
        while self.current<line.len() {
            if is_hbr_consonant(line[self.current]){
                new.push_str(self.get_color(&line).as_str())
            }
            else{
                new.push(line[self.current]);
                self.current += 1;
            }
        }
        new
    }
     fn get_color(&mut self,line:&Vec<char>)->String{
        let mut buf = String::new();
        while self.current<line.len()&&
            (is_hbr_consonant(line[self.current])||line[self.current]=='_')
        {
            buf.push(line[self.current]);
            self.current += 1;
        }
        if KEYWORDS.contains_key(buf.as_str()) {
            return Self::str_to_colored(buf.as_str(),KEYWORDS.get(buf.as_str()).unwrap());
        }
        buf
     }

    fn str_to_colored(buf:&str,color:&Color)->String{
        match color{
            Color::Yellow => {Colorize::yellow(buf).to_string()},
            Color::Green => {Colorize::green(buf).to_string()},
            Color::Blue=>{Colorize::blue(buf).to_string()},
            Color::Magenta=>{Colorize::magenta(buf).to_string()},
            Color::Orange=>{buf.truecolor(255, 165, 0).to_string()},
            Color::Cyan=>{Colorize::cyan(buf).to_string()}
        }
    }



}