use crossterm::*;

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
    pub fn set_fg_color(&self, fg: FgColor) -> style::SetForegroundColor {
        let fg_color = match fg {
            FgColor::Fixed => self.fg_fixed,
            FgColor::Player => self.fg_player,
            FgColor::Conflicting => self.fg_conflicting,
            FgColor::Highlighted => self.fg_highlighted,
        };
        style::SetForegroundColor(fg_color)
    }

    pub fn set_bg_color(&self, bg: BgColor) -> style::SetBackgroundColor {
        let bg_color = match bg {
            BgColor::Default => self.bg_default,
            BgColor::Highlighted => self.bg_highlighted,
        };
        style::SetBackgroundColor(bg_color)
    }
}
