use crossterm::{
    event::{self, Event, KeyEvent},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use std::io;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    
    execute!(io::stdout(), terminal::ScrollUp(10))?;

    // loop {
    //     let event = event::read()?;
    //     let Event::Key(KeyEvent { code, modifiers, .. }) = event else {
    //         continue
    //     }; 

    //     print!("{code}: {modifiers}\r\n");
    //     if code == event::KeyCode::Char('q') {
    //         break;
    //     }
    // }
    
    disable_raw_mode()?;
    execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
    Ok(())
}
