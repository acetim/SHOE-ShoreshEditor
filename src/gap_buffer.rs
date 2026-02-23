use std::fs;
use std::fs::File;
use std::io::ErrorKind;

pub struct GapBuffer{
    data:Vec<char>,
    gap_start:usize,
    gap_len:usize,
}

pub enum LeftRight{
    Left,
    Right,
}

impl GapBuffer{

    pub fn from_file(path:&str,capacity:usize)->Self{
        let data = match fs::read_to_string(path){
            Ok(string)=>string,
            Err(error)=>{
                match  error.kind(){
                    ErrorKind::NotFound=>{
                        File::create(path).expect("Problem creating the file");
                        String::new()
                    }
                    other_error=>{
                        panic!("{}",other_error);
                    }
                }
            }
        };
        let mut data_vec:Vec<char>=data.chars().collect();

        let current_char_count = data_vec.len();

        data_vec.resize(current_char_count + capacity, ' ');//make gap
        Self{
            data:data_vec,
            gap_start:current_char_count,
            gap_len:capacity
        }

}

    fn move_gap(&mut self, new_pos: usize) {
       if(new_pos <(self.data.len()-self.gap_len)+1) {
           while self.gap_start > new_pos { //if new_pos is to the left of curPos
               self.gap_start -= 1;
               self.data[self.gap_start + self.gap_len] = self.data[self.gap_start];
           }
           while self.gap_start < new_pos { //if new_pos is to the right of curPos
               self.data[self.gap_start] = self.data[self.gap_start + self.gap_len];
               self.gap_start += 1;
           }
       }
    }

    pub fn move_left_or_right(&mut self,m:LeftRight){
        match m {
            LeftRight::Right=>{
                if(self.gap_start>0) {//prevent underflow
                    self.move_gap(self.gap_start - 1)
                }
            }
            LeftRight::Left=>{
                self.move_gap(self.gap_start + 1)
            }
        }
    }

    fn grow(&mut self){
        let old_capacity = self.data.len();
        let new_capacity= old_capacity*2;
        let mut new_data =vec![' ';new_capacity];
        for i in 0..self.gap_start{
            new_data[i]=self.data[i];
        }
        let suffix_len = old_capacity - (self.gap_start + self.gap_len);
        let new_gap_len = new_capacity - self.gap_start - suffix_len;

        for i in 0..suffix_len {
            new_data[new_capacity - suffix_len + i] = self.data[old_capacity - suffix_len + i];
        }

        self.data = new_data;
        self.gap_len = new_gap_len;
    }

    pub fn insert(&mut self, c: char) {
        if self.gap_len == 0 {
            self.grow();
        }

        self.data[self.gap_start] = c;
        self.gap_start += 1;
        self.gap_len -= 1;
    }

    pub fn delete(&mut self){
        if(self.gap_start>0){
            self.gap_start-=1;
            self.gap_len+=1;

        }

    }

    pub fn calculate_cursor_pos(&self)->(u16,u16){
        let mut x = 0;
        let mut y = 0;


        for i in 0..self.gap_start {
            if self.data[i] == '\n' {
                y += 1;
                x = 0;
            } else {
                x += 1;
            }
        }
        (x as u16, y as u16)
    }

    pub fn to_string(&self) -> String{
        let pre_gap = self.data[..self.gap_start].iter();
        let post_gap = self.data[self.gap_start + self.gap_len..].iter();


        pre_gap.chain(post_gap).collect()
    }

    pub fn to_slices(&self) -> (String,String){
        let pre_gap = self.data[..self.gap_start].iter();
        let post_gap = self.data[self.gap_start + self.gap_len..].iter();


        (pre_gap.collect(),post_gap.collect())
    }

    pub fn data(&self) -> &Vec<char> {
        &self.data
    }
}