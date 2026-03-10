pub mod layout;
pub mod tabs;

use crate::app::App;
use ratatui::Frame;

pub fn render(frame: &mut Frame, app: &mut App) {
    layout::render(frame, app);
}
