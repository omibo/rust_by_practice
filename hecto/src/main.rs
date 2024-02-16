use std::io::{self, stdout}; 
use termion::input::TermRead; 
use termion::event::Key;
use termion::raw::IntoRawMode;

fn main() {            
    let _stdout = stdout().into_raw_mode().unwrap();
    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key{
                Key::Char(c) => {            
                    if c.is_control() {            
                        println!("{:?}\r", c as u8);            
                    } else {            
                        println!("{:?} ({})\r", c as u8, c);            
                    }
                }
                Key::Ctrl('q') => break,
                _ => println!("{:?}\r", key),
            },
            Err(err) => panic!("{}", err),
        }
    }
}