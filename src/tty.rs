use crossterm::*;
use std::io::Write;

pub struct TtyCtx<'w, W: Write>(&'w mut W);

impl<'w, W: Write> TtyCtx<'w, W> {
    pub fn init(out: &'w mut W) -> anyhow::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(
            out,
            event::EnableMouseCapture,
            cursor::Hide,
            terminal::EnterAlternateScreen
        )?;
        Ok(Self(out))
    }

    pub fn get_mut(&mut self) -> &mut W {
        self.0
    }
}

impl<'w, W: Write> Drop for TtyCtx<'w, W> {
    fn drop(&mut self) {
        if let Err(e) = execute!(
            self.0,
            terminal::LeaveAlternateScreen,
            cursor::Show,
            event::DisableMouseCapture
        ) {
            eprintln!("Failed to restore terminal state: {e}");
        }
        if let Err(e) = terminal::disable_raw_mode() {
            eprintln!("Failed to disable raw mode: {e}");
        }
        if let Err(e) = self.0.flush() {
            eprintln!("Failed to flush output: {e}");
        }
    }
}

pub struct TtyFrame<'f, 'w, W: Write>(&'f mut TtyCtx<'w, W>);

impl<'f, 'w, W: Write> TtyFrame<'f, 'w, W> {
    pub fn new(tty: &'f mut TtyCtx<'w, W>) -> Self {
        Self(tty)
    }

    pub fn get_mut(&mut self) -> &mut W {
        self.0.get_mut()
    }
}

impl<'f, 'w, W: Write> Drop for TtyFrame<'f, 'w, W> {
    fn drop(&mut self) {
        if let Err(e) = self.0.get_mut().flush() {
            eprintln!("Failed to flush output: {e}");
        }
    }
}
