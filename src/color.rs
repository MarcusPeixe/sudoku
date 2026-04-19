use crossterm::style;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FgColor {
    Fixed,
    Player,
    Conflicting,
    Highlighted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BgColor {
    Default,
    Highlighted,
}

pub struct Palette {
    pub fg_fixed: style::Color,
    pub fg_player: style::Color,
    pub fg_conflicting: style::Color,
    pub fg_highlighted: style::Color,
    pub bg_default: style::Color,
    pub bg_highlighted: style::Color,
}

impl Palette {
    pub fn set_colors(&self, fg: FgColor, bg: BgColor) -> style::SetColors {
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