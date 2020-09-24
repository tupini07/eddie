use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};

pub struct BasicAppLayout {
    pub Title: Rect,
    pub GroupContents: Rect,
    pub CommandOutput: Rect,
    pub ItemDiscription: Rect
}

pub fn create_layout<B: Backend>(f: &mut Frame<B>) -> BasicAppLayout {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
                .as_ref(),
        )
        .split(f.size());

    let chunks2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ]
                .as_ref(),
        )
        .split(chunks[1]);

    BasicAppLayout {
        Title: chunks[0],
        GroupContents: chunks2[0],
        CommandOutput: chunks2[1],
        ItemDiscription: chunks[2]
    }
}