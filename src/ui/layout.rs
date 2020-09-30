use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};

pub struct BasicAppLayout {
    pub Breadcrumbs: Rect,
    pub Title: Rect,
    pub GroupContents: Rect,
    pub CommandOutput: Rect,
    pub ItemDiscription: Rect,
    pub HelpContent: Rect
}

pub fn create_layout<B: Backend>(f: &mut Frame<B>) -> BasicAppLayout {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(55),
                Constraint::Percentage(20),
                Constraint::Percentage(3),
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
        .split(chunks[2]);

    BasicAppLayout {
        Breadcrumbs: chunks[0],
        Title: chunks[1],
        GroupContents: chunks2[0],
        CommandOutput: chunks2[1],
        ItemDiscription: chunks[3],
        HelpContent: chunks[4]
    }
}