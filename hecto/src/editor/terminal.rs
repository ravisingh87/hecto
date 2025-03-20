use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode,enable_raw_mode,size,Clear,ClearType};
use std::io::stdout;

pub struct Terminal{}

impl Terminal{
    pub fn termnate() -> Result<(),std::io::Error>{
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(),std::io::Error>{
        enable_raw_mode()?;
        let _ = Self::clear_screen()?;
        let _ = Self::move_cursor(0,0)?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(),std::io::Error>{
        execute!(stdout(),Clear(ClearType::All))?;
        Ok(())
    }
    pub fn move_cursor(x:u16,y:u16) -> Result<(),std::io::Error>{
        execute!(stdout(),MoveTo(x,y))?;
        Ok(())
    }
    pub fn size() -> Result<(u16,u16),std::io::Error>{
        size()
    }
}