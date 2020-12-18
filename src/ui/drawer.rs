use tui::backend::Backend;
use tui::Frame;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};

use crate::ui::layout::BasicAppLayout;
use crate::ui::state::UiState;

pub fn draw_tui<B: Backend>(frame: &mut Frame<B>, layout: &BasicAppLayout, state: &mut UiState) {
    { // render breadcrumbs
        let block = Block::default().borders(Borders::BOTTOM | Borders::TOP);
        let flat_bread: String = state
            .breadcrumbs
            .iter()
            .map(|e| e.name.clone())
            .collect::<Vec<String>>()
            .join(" / ");
        
        let bc_text = format!("{} > {}", &crate::APP_CONFIG.eddie_config.ship_name, &flat_bread);
        let paragraph = Paragraph::new(bc_text).block(block);
        frame.render_widget(paragraph, layout.breadcrumbs);
    }

    { // Render title
        let block = Block::default().borders(Borders::ALL);
        let paragraph = Paragraph::new(Span::from(state.title))
            .style(Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::White)
                .bg(Color::DarkGray))
            .block(block);
        frame.render_widget(paragraph, layout.title);
    }

    { // Render list items
        let block = Block::default().title("Group items").borders(Borders::ALL);

        let items: Vec<_> = state.group_items_state
            .items
            .iter()
            .map(|i| {
                let lines = vec![Spans::from(*i)];
                ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::Blue))
            }).collect();

        let lsst = List::new(items).block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">> ");
        frame.render_stateful_widget(lsst, layout.group_contents, &mut state.group_items_state.state);
    }

    { // render Command output
        let block = Block::default().title("Command outputs").borders(Borders::ALL);
        let paragraph = Paragraph::new(state.command_output.as_str()).block(block);
        frame.render_widget(paragraph, layout.command_output);
    }

    { // Render Item description
        let block = Block::default().title("Item description").borders(Borders::ALL);
        let paragraph = Paragraph::new(state.description).wrap(Wrap { trim: true }).block(block);
        frame.render_widget(paragraph, layout.item_description);
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
        frame.render_widget(paragraph, layout.help_content);
    }
}
