use tui::backend::Backend;
use tui::Frame;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::ui::layout::BasicAppLayout;
use crate::ui::state::UiState;

pub fn draw_tui<B: Backend>(frame: &mut Frame<B>, layout: &BasicAppLayout, state: &mut UiState) {
    { // render breadcrumbs
        let block = Block::default().borders(Borders::BOTTOM | Borders::TOP);
        let flat_bread: String = state
            .current_breadcrumbs
            .iter()
            .map(|e| e.name.clone())
            .collect::<Vec<String>>()
            .join(" > ");
        let paragraph = Paragraph::new(flat_bread.as_str()).block(block);
        frame.render_widget(paragraph, layout.Breadcrumbs);
    }

    { // Render title
        let block = Block::default().borders(Borders::ALL);
        let paragraph = Paragraph::new(Span::from(state.current_title.as_str()))
            .style(Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::White)
                .bg(Color::DarkGray))
            .block(block);
        frame.render_widget(paragraph, layout.Title);
    }

    { // Render list items
        let block = Block::default().title("Group items").borders(Borders::ALL);

        let items: Vec<_> = state.current_group_items_state
            .items
            .iter()
            .map(|i| {
                let mut lines = vec![Spans::from(*i)];
                ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::Blue))
            }).collect();

        let lsst = List::new(items).block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">> ");
        frame.render_stateful_widget(lsst, layout.GroupContents, &mut state.current_group_items_state.state);
    }

    { // render Command output
        let block = Block::default().title("Command outputs").borders(Borders::ALL);
        let paragraph = Paragraph::new(state.current_command_output.as_str()).block(block);
        frame.render_widget(paragraph, layout.CommandOutput);
    }

    { // Render Item description
        let block = Block::default().title("Item description").borders(Borders::ALL);
        let paragraph = Paragraph::new(state.current_description.as_str()).block(block);
        frame.render_widget(paragraph, layout.ItemDiscription);
    }


    { // render help message
        let block = Block::default().borders(Borders::NONE);
        let paragraph = Paragraph::new(Span::from("TAB to select next / Shift + TAB to select previous / RETURN to select / BACKSPACE to go back"))
            .style(Style::default()
                       .add_modifier(Modifier::BOLD)
                       .fg(Color::Yellow)
                   // .bg(Color::DarkGray)
            )
            .block(block);
        frame.render_widget(paragraph, layout.HelpContent);
    }
}
