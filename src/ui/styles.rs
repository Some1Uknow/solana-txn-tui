use ratatui::style::{Color, Modifier, Style};

pub const PRIMARY_COLOR: Color = Color::Cyan;
pub const SECONDARY_COLOR: Color = Color::Blue;
pub const SUCCESS_COLOR: Color = Color::Green;
pub const ERROR_COLOR: Color = Color::Red;
#[allow(dead_code)]
pub const WARNING_COLOR: Color = Color::Yellow;
pub const TEXT_COLOR: Color = Color::White;
pub const DIM_COLOR: Color = Color::Gray;
pub const BG_COLOR: Color = Color::Black;

pub const PRIMARY_STYLE: Style = Style::new().fg(PRIMARY_COLOR).bg(BG_COLOR);

pub const SECONDARY_STYLE: Style = Style::new().fg(SECONDARY_COLOR).bg(BG_COLOR);

pub const TEXT_STYLE: Style = Style::new().fg(TEXT_COLOR).bg(BG_COLOR);

pub const DIM_STYLE: Style = Style::new().fg(DIM_COLOR).bg(BG_COLOR);

pub const SUCCESS_STYLE: Style = Style::new()
    .fg(SUCCESS_COLOR)
    .bg(BG_COLOR)
    .add_modifier(Modifier::BOLD);

pub const ERROR_STYLE: Style = Style::new()
    .fg(ERROR_COLOR)
    .bg(BG_COLOR)
    .add_modifier(Modifier::BOLD);

#[allow(dead_code)]
pub const WARNING_STYLE: Style = Style::new()
    .fg(WARNING_COLOR)
    .bg(BG_COLOR)
    .add_modifier(Modifier::BOLD);

pub const HEADER_STYLE: Style = Style::new()
    .fg(PRIMARY_COLOR)
    .bg(BG_COLOR)
    .add_modifier(Modifier::BOLD);

pub const SELECTED_STYLE: Style = Style::new()
    .fg(BG_COLOR)
    .bg(PRIMARY_COLOR)
    .add_modifier(Modifier::BOLD);

pub const HINT_STYLE: Style = Style::new()
    .fg(DIM_COLOR)
    .bg(BG_COLOR)
    .add_modifier(Modifier::ITALIC);
