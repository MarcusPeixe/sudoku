use crossterm::*;
use crate::{color, game};

pub trait Renderer {
    fn render(
        &self,
        stdout: &mut impl std::io::Write,
        game: &game::GameState,
    ) -> anyhow::Result<()>;

    fn render_final(
        &self,
        stdout: &mut impl std::io::Write,
        game: &game::GameState,
    ) -> anyhow::Result<()>;
}

pub struct LargeRenderer {
    palette: color::Palette,
}

impl LargeRenderer {
    pub fn new(palette: color::Palette) -> Self {
        Self { palette }
    }

    fn render_cell(
        &self,
        stdout: &mut impl std::io::Write,
        cell: game::Cell,
        line: usize,
        fixed: bool,
        selected_number: Option<game::Number>,
        invalid: &[bool; 9],
    ) -> anyhow::Result<()> {
        static DIGIT_PATTERNS: [[&str; 3]; 9] = [
            ["   ▄   ", "  ▀█   ", "  ▄█▄  "],
            [" ▄▄▄▄▄ ", " ▄▄▄▄█ ", " █▄▄▄▄ "],
            [" ▄▄▄▄▄ ", "  ▄▄▄█ ", " ▄▄▄▄█ "],
            [" ▄   ▄ ", " █▄▄▄█ ", "     █ "],
            [" ▄▄▄▄▄ ", " █▄▄▄▄ ", " ▄▄▄▄█ "],
            [" ▄▄▄▄▄ ", " █▄▄▄▄ ", " █▄▄▄█ "],
            [" ▄▄▄▄▄ ", " ▀   █ ", "     █ "],
            [" ▄▄▄▄▄ ", " █▄▄▄█ ", " █▄▄▄█ "],
            [" ▄▄▄▄▄ ", " █▄▄▄█ ", " ▄▄▄▄█ "],
        ];
        match cell {
            game::Cell::Empty { mask } => {
                for i in 0..3 {
                    let number = game::Number::new((line * 3 + i + 1) as u8)
                        .expect("`line` should be in 0..3");
                    if mask.contains(number) {
                        let color = if Some(number) == selected_number {
                            color::FgColor::Highlighted
                        } else if invalid[number.get_idx()] {
                            color::FgColor::Conflicting
                        } else {
                            color::FgColor::Player
                        };
                        queue!(stdout, self.palette.set_fg_color(color))?;
                        write!(stdout, " {}", number.get())?;
                    } else {
                        write!(stdout, "  ")?;
                    }
                }
                write!(stdout, " ")?;
            }
            game::Cell::Filled { value } => {
                let color = if fixed {
                    color::FgColor::Fixed
                } else if Some(value) == selected_number {
                    color::FgColor::Highlighted
                } else if invalid[value.get_idx()] {
                    color::FgColor::Conflicting
                } else {
                    color::FgColor::Player
                };
                queue!(stdout, self.palette.set_fg_color(color))?;
                write!(stdout, "{}", DIGIT_PATTERNS[value.get_idx()][line])?;
            }
        }
        Ok(())
    }

    fn render(
        &self,
        stdout: &mut impl std::io::Write,
        board: &game::Board,
        fixed: &[[bool; 9]; 9],
        invalid: &[[[bool; 9]; 9]; 9],
        (pi, pj): (usize, usize),
        selected_number: Option<game::Number>,
    ) -> anyhow::Result<()> {
        #[allow(clippy::needless_range_loop)]
        for i in 0..9 {
            for line in 0..3 {
                for j in 0..9 {
                    let color = if pi == i || pj == j || (pi / 3 == i / 3 && pj / 3 == j / 3) {
                        color::BgColor::Highlighted
                    } else {
                        color::BgColor::Default
                    };
                    queue!(stdout, self.palette.set_bg_color(color))?;
                    self.render_cell(
                        stdout,
                        board.cells[i][j],
                        line,
                        fixed[i][j],
                        selected_number,
                        &invalid[i][j],
                    )?;
                }
                queue!(stdout, style::ResetColor)?;
                write!(stdout, "\r\n")?;
            }
        }
        Ok(())
    }
}

impl Renderer for LargeRenderer {
    fn render(
        &self,
        stdout: &mut impl std::io::Write,
        game: &game::GameState,
    ) -> anyhow::Result<()> {
        self.render(
            stdout,
            &game.board,
            &game.fixed,
            &game.invalid,
            game.player_pos,
            game.selected_number,
        )
    }

    fn render_final(
        &self,
        stdout: &mut impl std::io::Write,
        game: &game::GameState,
    ) -> anyhow::Result<()> {
        self.render(
            stdout,
            &game.board,
            &game.fixed,
            &game.invalid,
            (9, 9),
            None,
        )
    }
}
