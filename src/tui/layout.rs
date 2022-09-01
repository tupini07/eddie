use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};

pub struct BasicAppLayout {
    pub breadcrumbs: Rect,
    pub title: Rect,
    pub group_contents: Rect,
    pub command_output: Rect,
    pub item_description: Rect,
    pub help_content: Rect
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
        breadcrumbs: chunks[0],
        title: chunks[1],
        group_contents: chunks2[0],
        command_output: chunks2[1],
        item_description: chunks[3],
        help_content: chunks[4]
    }
}