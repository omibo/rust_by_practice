use crate::Terminal;
use termion::event::Key;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        loop {            
            if let Err(error) = self.refresh_screen() {    
                Terminal::clear_screen();       
                panic!("{error}");
            }
            if self.should_quit {
                break;
            }
            else {
                self.draw_rows();            
                print!("{}", termion::cursor::Goto(1, 1));
            }
            if let Err(error) = self.process_keypress() { 
                Terminal::clear_screen();           
                panic!("{error}");
            }
        }
    }

    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height-1 {
            println!("~\r");
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {            
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        if self.should_quit {            
            println!("Goodbye.\r");            
        }
        else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }    
        Terminal::flush()            
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;            
            match pressed_key {            
                Key::Ctrl('q') => self.should_quit = true,            
                _ => (),            
            }            
            Ok(())
    }
}