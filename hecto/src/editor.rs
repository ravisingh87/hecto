use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;
mod terminal;
use terminal::{Terminal,Size,Position};

pub struct Editor{
    should_quit:bool,
}

impl Editor{
    pub const fn default() -> Self{
        Self { should_quit: false }
    }

    pub fn run(&mut self){
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::termnate().unwrap();
        result.unwrap();
    }
    
    fn repl(&mut self)-> Result<(),Error>{
        loop {
         self.refresh_screen()?;
         if self.should_quit{
            break;
         }   
         let event = read()?;
         self.evaluate_event(&event);
        }
        Ok(()) // Here we disable raw mode again
    }

    fn evaluate_event(&mut self,event: &Event){
        if let Key(KeyEvent{
            code,modifiers,..
        }) = event{
            match code{
                Char('q') if *modifiers == KeyModifiers::CONTROL =>{
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self)-> Result<(),Error>{
        Terminal::hide_cursor()?;
        if self.should_quit{
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        }else{
            Self::draw_row()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_row()-> Result<(),Error>{
        // let height = Terminal::size()?.1;
        let Size { height, ..} = Terminal::size()?;
        for cursor_row in 0..height{
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if cursor_row + 1 < height{
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
} 