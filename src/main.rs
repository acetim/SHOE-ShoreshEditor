
mod gap_buffer;
mod viewer;
mod text_editor;
mod editor;
mod color_tokenizer;
use std::env;
use std::process::exit;
use crate::text_editor::TextEditor;
use crate::color_tokenizer::ColorTokenizer;
fn main()  {
    //TODO FIX SCREEN FLICKERING AND OPTIMIZE RENDER() LATER

    let args: Vec<String> = env::args().collect();
    if(args.len()<2){
        println!("{}", "אנא צרף את שם הקובץ אותו תרצה לפתוח או ליצור".chars().rev().collect::<String>());
        exit(1);
    }
    let file_path = args[1].as_str();

    let mut text_editor= TextEditor::new(file_path);
    text_editor.start_text_editor().expect("error encountered while trying to open file");

}
fn print(){
    let mut colorizer =ColorTokenizer::new();
    let str = String::from("ויאמר עידן");
    let vec: Vec<char>=str.chars().collect();
    print!("{}",colorizer.colorize(&vec));
}
