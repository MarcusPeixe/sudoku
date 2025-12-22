use crossterm::{
    cursor, event::{self, Event, KeyEvent}, execute, terminal::{self, disable_raw_mode, enable_raw_mode}
};
use std::io;

struct Number(u8);

impl Number {
    fn new(value: u8) -> Option<Self> {
        (1..=9).contains(&value).then_some(Self(value))
    }
}

enum Cell {
    Empty { pencil_mask: u16 },
    Filled { value: u8 },
}

fn main() -> io::Result<()> {
    let (w, h) = terminal::size()?;
    if w < 27 || h < 27 {
        eprintln!("Terminal size must be at least 27x27");
        return Ok(());
    }

    enable_raw_mode()?;

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
    Ok(())
}
