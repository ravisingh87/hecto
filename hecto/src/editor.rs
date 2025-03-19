use std::io::{self,Read};  // from standard library importing Input and output stuff(io)
use crossterm::terminal::{enable_raw_mode,disable_raw_mode};

pub struct Editor{}

impl Editor{
    pub fn default() -> Self{
        Editor{}
    }
    pub fn run(&self){
        enable_raw_mode().unwrap(); // We enter raw mode with this method
    for b in io::stdin().bytes(){
        // These two lines basically print out the character that was read
        match b{
            Ok(b)=> {
                let c = b as char;
                if c.is_control(){
                    println!("Binary: {0:08b} ASCII: {0:#03} \r",b);
                }else{
                    println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r",b,c);
                }
                if c == 'q'{
                    break;
                }
            }
            Err(err)=> println!("{}",err),
        }
    }
    disable_raw_mode().unwrap(); // Here we disable raw mode again
    }
} 