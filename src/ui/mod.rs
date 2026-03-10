pub mod layout;

use ratatui::Frame;
use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    layout::render(frame, app);
}
