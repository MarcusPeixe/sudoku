mod color;
mod game;
mod renderer;
mod tty;

use crossterm::event::*;
use crossterm::*;

use crossterm::tty::IsTty;
use renderer::Renderer;
use std::io::{self, Write};

use clap::Parser;

/// Sudoku game in terminal
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Play on smaller board
    #[arg(short, long)]
    small: bool,
}

fn main() -> anyhow::Result<()> {
    // TODO: select renderer based on `--small` flag
    // let args = Args::parse();

    let palette = color::Palette {
        fg_fixed: style::Color::White,
        fg_player: style::Color::Blue,
        fg_conflicting: style::Color::Red,
        fg_highlighted: style::Color::Green,
        bg_default: style::Color::Reset,
        bg_highlighted: style::Color::Black,
    };

    let mut game_state = game::GameState::new();
    let renderer = renderer::LargeRenderer::new(palette);

    for i in 0..9 {
        for j in 0..9 {
            let d = (i * 3 + i / 3 + j) % 9 + 1;
            let num = game::Number::new(d as u8).expect("`d` should be in 1..=9");
            game_state.board.cells[i][j] = game::Cell::new_filled(num);
            game_state.fixed[i][j] = (i + j) % 2 == 0;
        }
    }
    game_state.board.cells[8][8] = game::Cell::new_with_mask(
        game::PencilMask::new()
            .set(game::Number::new(1).unwrap())
            .set(game::Number::new(8).unwrap()),
    );
    game_state.fixed[8][8] = false;
    game_state.update_invalid();

    let mut stdout = io::stdout();

    if !stdout.is_tty() {
        anyhow::bail!("Standard output must be a terminal");
    }

    let (mut w, mut h) = terminal::size()?;
    let mut tty = tty::TtyCtx::init(&mut stdout)?;

    queue!(tty.get_mut(), cursor::MoveTo(0, 0))?;
    queue!(tty.get_mut(), terminal::Clear(terminal::ClearType::All))?;

    if w < 63 || h < 29 {
        // TODO: delegate terminal size check to renderer
        write!(
            tty.get_mut(),
            "Terminal size must be at least 63x29 (currently {w}x{h})\r\n"
        )?;
    } else {
        renderer.render(tty.get_mut(), &game_state)?;
    }

    loop {
        let event = event::read()?; // This blocks, so only clear screen afterwards to avoid losing writes to screen

        match event {
            Event::Resize(new_w, new_h) => {
                w = new_w;
                h = new_h;
            }
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Moved,
                column,
                row,
                ..
            }) => {
                let i = row as usize / 3;
                let j = column as usize / 7;
                if i < 9 && j < 9 {
                    game_state.player_pos = (i, j);
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up | KeyCode::Char('w'),
                ..
            }) => {
                game_state.player_pos.0 = game_state.player_pos.0.saturating_sub(1);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down | KeyCode::Char('s'),
                ..
            }) => {
                game_state.player_pos.0 = (game_state.player_pos.0 + 1).min(8);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left | KeyCode::Char('a'),
                ..
            }) => {
                game_state.player_pos.1 = game_state.player_pos.1.saturating_sub(1);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right | KeyCode::Char('d'),
                ..
            }) => {
                game_state.player_pos.1 = (game_state.player_pos.1 + 1).min(8);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => {
                break;
            }
            _ => {}
        }

        let mut frame = tty::TtyFrame::new(&mut tty);

        queue!(frame.get_mut(), cursor::MoveTo(0, 0))?;
        queue!(frame.get_mut(), terminal::Clear(terminal::ClearType::All))?;

        if w < 63 || h < 29 {
            // TODO: delegate terminal size check to renderer
            write!(
                frame.get_mut(),
                "Terminal size must be at least 63x29 (currently {w}x{h})\r\n"
            )?;
        } else {
            renderer.render(frame.get_mut(), &game_state)?;
        }

        write!(frame.get_mut(), "[{w}x{h}] {event:?}\r\n")?;
    }

    drop(tty); // restore terminal state before printing final board
    renderer.render_final(&mut stdout, &game_state)?;

    Ok(())
}
