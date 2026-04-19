use crate::{color, game};

trait Renderer {
    fn render(&self, stdout: &mut impl std::io::Write, game: &game::GameState) -> anyhow::Result<()>;
}

struct LargeRenderer {
    palette: color::Palette,
}

impl LargeRenderer {
    fn new(palette: color::Palette) -> Self {
        Self { palette }
    }
}