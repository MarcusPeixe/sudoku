use crossterm::{
    cursor, event::{self, Event, KeyEvent, MouseEvent}, execute, queue, style, terminal
};
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Number(u8);

impl Number {
    fn new(value: u8) -> Option<Self> {
        (0..9).contains(&value).then_some(Self(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PencilMask(u16);

impl PencilMask {
    fn new() -> Self {
        Self(0)
    }

    fn set(self, number: Number) -> Self {
        Self(self.0 | (1 << number.0))
    }

    fn reset(self, number: Number) -> Self {
        Self(self.0 & !(1 << number.0))
    }

    fn contains(&self, number: Number) -> bool {
        self.0 & (1 << number.0) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty { mask: PencilMask },
    Filled { value: Number },
}

impl Cell {
    fn new_empty() -> Self {
        Self::Empty { mask: PencilMask::new() }
    }

    fn new_with_mask(mask: PencilMask) -> Self {
        Self::Empty { mask }
    }

    fn new_filled(value: Number) -> Self {
        Self::Filled { value }
    }

    fn is_empty(&self) -> bool {
        matches!(self, Self::Empty { .. })
    }

    fn is_filled(&self) -> bool {
        matches!(self, Self::Filled { .. })
    }

    fn render(&self, stdout: &mut impl Write, line: usize) -> anyhow::Result<()> {
        match self {
            Self::Empty { mask } => {
                for i in 0..3 {
                    let number = Number::new((line * 3 + i) as u8).expect("`line` should be in 0..3");
                    if mask.contains(number) {
                        write!(stdout, " {}", number.0 + 1)?;
                    } else {
                        write!(stdout, "  ")?;
                    }
                }
                write!(stdout, " ")?;
            }
            Self::Filled { value } => {
                write!(stdout, "{}", DIGIT_PATTERNS[value.0 as usize][line])?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FgColor {
    Fixed,
    Player,
    Conflicting,
    Highlighted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BgColor {
    Default,
    Highlighted,
}

struct Palette {
    fg_fixed: style::Color,
    fg_player: style::Color,
    fg_conflicting: style::Color,
    fg_highlighted: style::Color,
    bg_default: style::Color,
    bg_highlighted: style::Color,
}

impl Palette {
    fn set_colors(&self, fg: FgColor, bg: BgColor) -> style::SetColors {
        let fg_color = match fg {
            FgColor::Fixed => self.fg_fixed,
            FgColor::Player => self.fg_player,
            FgColor::Conflicting => self.fg_conflicting,
            FgColor::Highlighted => self.fg_highlighted,
        };
        let bg_color = match bg {
            BgColor::Default => self.bg_default,
            BgColor::Highlighted => self.bg_highlighted,
        };
        style::SetColors(style::Colors::new(fg_color, bg_color))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Board {
    cells: [[Cell; 9]; 9],
}

impl Board {
    fn new() -> Self {
        Self { cells: [[Cell::new_empty(); 9]; 9] }
    }

    fn get_cell(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells[row][col] = cell;
    }

    fn render(&self, stdout: &mut impl Write) -> anyhow::Result<()> {
        self.render_with_player(stdout, (9, 9))  // out of bounds
    }

    fn render_with_player(&self, stdout: &mut impl Write, player_pos: (usize, usize)) -> anyhow::Result<()> {
        let (pi, pj) = player_pos;
        queue!(stdout, cursor::MoveUp(27))?;
        for i in 0..27 {
            let line = i % 3;
            let i = i / 3;
            for j in 0..9 {
                let color = {
                    if pi == i || pj == j || (pi / 3 == i / 3 && pj / 3 == j / 3) {
                        "\x1b[40m"
                    } else {
                        "\x1b[49m"
                    }
                };
                write!(stdout, "{}", color)?;
                self.cells[i][j].render(stdout, line)?;
            }
            write!(stdout, "\x1b[0m\r\n")?;
        }
        stdout.flush()?;
        Ok(())
    }
}

struct GameState {
    board: Board,
    player_pos: (usize, usize),
    palette: Palette,
}

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
    // let args = Args::parse();

    let (w, h) = terminal::size()?;
    if w < 63 || h < 27 {
        anyhow::bail!("Terminal size must be at least 63x27 (currently {}x{})", w, h);
    }

    let palette = Palette {
        fg_fixed: style::Color::Grey,
        fg_player: style::Color::Blue,
        fg_conflicting: style::Color::Red,
        fg_highlighted: style::Color::Green,
        bg_default: style::Color::Reset,
        bg_highlighted: style::Color::Black,
    };

    let mut stdout = io::stdout();
    for _ in 0..27 {
        writeln!(stdout)?;
    }

    execute!(stdout, event::EnableMouseCapture, cursor::Hide)?;
    terminal::enable_raw_mode()?;

    let mut board = Board::new();

    // board.set_cell(0, 0, Cell::new_filled(Number(0)));
    // board.set_cell(0, 1, Cell::new_filled(Number(1)));
    // board.set_cell(0, 2, Cell::new_filled(Number(2)));
    // board.set_cell(1, 0, Cell::new_filled(Number(3)));
    // board.set_cell(1, 1, Cell::new_filled(Number(4)));
    // board.set_cell(1, 2, Cell::new_filled(Number(5)));
    // board.set_cell(2, 0, Cell::new_filled(Number(6)));
    // board.set_cell(2, 1, Cell::new_filled(Number(7)));
    // board.set_cell(2, 2, Cell::new_filled(Number(8)));

    // board.set_cell(1, 7, Cell::new_with_mask(PencilMask(0b101010101)));

    // board.set_cell(4, 4, Cell::new_with_mask(PencilMask(0b111101111)));

    // board.set_cell(7, 1, Cell::new_with_mask(PencilMask(0b010101010)));

    // board.set_cell(7, 7, Cell::new_with_mask(PencilMask(0b111111111)));

    for i in 0..9 {
        for j in 0..9 {
            let d = (i * 3 + i / 3 + j) % 9;
            board.set_cell(i, j, Cell::new_filled(Number(d as u8)));
        }
    }

    let mut player_pos = (0, 0);
    loop {
        board.render_with_player(&mut stdout, player_pos)?;

        let event = event::read()?;

        write!(stdout, "{event:?}\r\n")?;

        execute!(stdout, cursor::MoveUp(1))?;

        let Event::Key(KeyEvent { code, modifiers, .. }) = event else {
            continue
        };

        match code {
            event::KeyCode::Up | event::KeyCode::Char('w') => player_pos.0 = player_pos.0.saturating_sub(1),
            event::KeyCode::Down | event::KeyCode::Char('s') => player_pos.0 = (player_pos.0 + 1).min(8),
            event::KeyCode::Left | event::KeyCode::Char('a') => player_pos.1 = player_pos.1.saturating_sub(1),
            event::KeyCode::Right | event::KeyCode::Char('d') => player_pos.1 = (player_pos.1 + 1).min(8),
            event::KeyCode::Char('q') => { break; }
            _ => {}
        }
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, event::DisableMouseCapture, cursor::Show)?;
    Ok(())
}
