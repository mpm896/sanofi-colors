// Colors - RGB
// Miracle Purple -- 35 0 76
// Chasing Purple -- 122 0 230
// Support Gray -- 244 242 246
// Support White -- 255 255 255
// Font Black -- 0 0 0
// Font white -- 245 245 245

// Colors - 255 Indexed (closest to true Sanofi colors)
// Miracle Purple -- 17
// Chasing Purple -- 92
// Support Gray -- 254
// Support White -- 231
// Font Black -- 16
// Font white -- 255

use ratatui::style::Color;
use strum::{Display, EnumIter};

#[derive(Clone, Copy, EnumIter, Debug, Display)]
pub enum Colors {
    MiraclePurple,
    ChasingPurple,
    SupportGray,
    SupportWhite,
    FontBlack,
    FontWhite
}

impl Colors {
    pub fn get_color(&self) -> Color {
        match self {
            Self::MiraclePurple => Color::Indexed(17),
            Self::ChasingPurple => Color::Indexed(92),
            Self::SupportGray => Color::Indexed(254),
            Self::SupportWhite => Color::Indexed(231),
            Self::FontBlack => Color::Indexed(16),
            Self::FontWhite => Color::Indexed(255)
        }
    }

}