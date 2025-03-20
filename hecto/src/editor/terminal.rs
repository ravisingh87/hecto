use crossterm::cursor::{Hide,MoveTo,Show};
use crossterm::{queue,Command};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode,enable_raw_mode,size,Clear,ClearType};
use std::io::{stdout,Error,Write};
use core::fmt::Display;

#[derive(Clone, Copy)]
pub struct Size{
    pub height:u16,
    pub width:u16,
}

#[derive(Clone, Copy)]
pub struct Position{
    pub x:u16,
    pub y:u16,
}

pub struct Terminal;

impl Terminal{
    pub fn termnate() -> Result<(),Error>{
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    
    pub fn initialize() -> Result<(),Error>{
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{x:0,y:0})?;
        Self::execute()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(),Error>{
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(),Error>{
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cursor_to(position:Position) -> Result<(),Error>{
        Self::queue_command(MoveTo(position.x,position.y))?;
        Ok(())
    }

    pub fn hide_cursor()-> Result<(),Error>{
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_cursor()-> Result<(),Error>{
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn print<T:Display>(string: T) -> Result<(),Error>{
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn size() -> Result<Size,Error>{
        let (width,height) = size()?;
        Ok(Size{height,width})
    }

    pub fn execute() -> Result<(),Error>{
        stdout().flush()?;
        Ok(())
    }

    pub fn queue_command<T:Command>(command:T)-> Result<(),Error>{
        queue!(stdout(),command)?;
        Ok(())
    }
}