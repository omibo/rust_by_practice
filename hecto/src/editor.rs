use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

pub struct Position {
    pub x: usize, 
    pub y: usize,
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
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    fn draw_welcome_message(&self) {            
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);            
        let width: usize = self.terminal.size().width as usize;            
        let len = welcome_message.len();            
        let padding: usize = width.saturating_sub(len) / 2;            
        let spaces: String = " ".repeat(padding.saturating_sub(1));            
        welcome_message = format!("~{}{}", spaces, welcome_message);            
        welcome_message.truncate(width);            
        println!("{}\r", welcome_message);            
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..self.terminal.size().height-1 {
            Terminal::clear_current_line();
            if row == height / 3 {            
                self.draw_welcome_message();         
            } else {            
                println!("~\r");            
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {  
        Terminal::cursor_hide();          
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if self.should_quit {    
            Terminal::clear_screen();        
            println!("Goodbye.\r");            
        }
        else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()            
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;            
            match pressed_key {            
                Key::Ctrl('q') => self.should_quit = true,
                Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),        
                _ => (),            
            }            
            Ok(())
    }

    fn move_cursor(&mut self, key: Key) {            
        let Position { mut y, mut x } = self.cursor_position;            
        match key {            
            Key::Up => y = y.saturating_sub(1),            
            Key::Down => y = y.saturating_add(1),            
            Key::Left => x = x.saturating_sub(1),            
            Key::Right => x = x.saturating_add(1),            
            _ => (),            
        }            
        self.cursor_position = Position { x, y }            
    }
}