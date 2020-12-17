use core::fmt;
use std::borrow::Borrow;

use crate::config_reader::config_structs::ConfigNode;
use crate::ui::util::StatefulList;

pub struct UiState<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub breadcrumbs: Vec<&'a ConfigNode>,
    pub group_items_state: StatefulList<&'a str>,
    pub group_items: Vec<&'a ConfigNode>,
    pub current_node: &'a ConfigNode,
    pub root_node: &'a ConfigNode,
    pub command_output: String,
    pub need_redraw: bool,
}

impl<'a> UiState<'a> {
    pub fn new(root_node: &'a ConfigNode) -> UiState<'a> {
        let mut state = UiState {
            title: "",
            description: "",
            breadcrumbs: vec![],
            current_node: root_node,
            group_items: vec![],
            group_items_state: StatefulList::new(),
            command_output: "".to_string(),
            root_node: root_node,
            need_redraw: false,
        };

        state.set_config_for_node(root_node);
        state
    }

    pub fn set_config_for_node(&mut self, node: &'a ConfigNode) {
        self.title = &node.name;
        self.description = &node.description;
        self.current_node = node;

        if let Some(children) = &node.children {
            self.group_items = children.iter().collect();

            let items_for_state = children
                .iter()
                .map(|e| e.name.as_str())
                .collect::<Vec<&str>>();

            self.group_items_state = StatefulList::with_items(items_for_state);

            // set the first element of the state as selected
            self.group_items_state.next();

            // set description to that of the first element of the group
            self.update_description();
        } else {
            self.current_node = self.root_node;
            self.group_items_state = StatefulList::new();
        }
    }

    pub fn get_selected_node(&self) -> Option<&'a ConfigNode> {
        let selected_i = self.group_items_state.state.selected()?;
        Some(self.group_items.get(selected_i)?)
    }

    pub fn update_description(&mut self) -> Option<()> {
        let selected_node = self.get_selected_node()?;
        self.description = &selected_node.description;

        Some(())
    }

    pub fn enter_selected_node(&mut self) -> Option<()> {
        let selected_node = self.get_selected_node()?;

        if !selected_node.is_leaf() {
            self.breadcrumbs.push(self.current_node);
            self.set_config_for_node(selected_node);
        }

        Some(())
    }

    pub fn exit_current_node(&mut self) -> Option<()> {
        let previous_node;
        if !self.breadcrumbs.is_empty() {
            previous_node = self.breadcrumbs.pop()?;
            self.set_config_for_node(previous_node);
        }

        Some(())
    }
}

impl<'a> fmt::Debug for UiState<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UiState")
            .field("title", &self.title)
            .field("description", &self.description)
            .field("breadcrumbs", &self.breadcrumbs)
            .field("current_node", &self.current_node)
            .field("command_output", &self.command_output)
            .field(
                "group_items_state",
                &self.group_items_state.state.selected(),
            )
            .field("group_items_state_list", &self.group_items_state.items)
            .field("need_redraw", &self.need_redraw)
            .finish()
    }
}
