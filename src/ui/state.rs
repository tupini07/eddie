use core::fmt;

use crate::config_reader::config_structs::ConfigNode;
use crate::ui::util::StatefulList;

pub struct UiState<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub breadcrumbs: Vec<&'a ConfigNode>,
    pub group_items: Vec<&'a ConfigNode>,
    pub group_items_state: StatefulList<&'a str>,
    pub command_output: String,
}

impl<'a> UiState<'a> {
    pub fn new() -> UiState<'a> {
        UiState {
            title: "",
            description: "",
            breadcrumbs: vec![],
            group_items: vec![],
            group_items_state: StatefulList::new(),
            command_output: "".to_string(),
        }
    }

    pub fn set_config_for_node(&mut self, node: &'a ConfigNode) {
        self.title = &node.name;
        self.description = &node.description;

        if let Some(children) = &node.children {
            self.group_items = children.iter().collect();

            let mut items_for_state = children
                .iter()
                .map(|e| e.name.as_str())
                .collect::<Vec<&str>>();

            // sort elements of group alphabetically to make it nicer
            items_for_state.sort_by_key(|e| e.to_lowercase());

            self.group_items_state = StatefulList::with_items(items_for_state);

            // finally, always set the first element of the state as selected
            self.group_items_state.next();
        } else {
            self.group_items = vec![];
            self.group_items_state = StatefulList::new();
        }

        self.command_output = "potato".to_string();
    }

    fn get_selected_node(&self) -> Option<&'a ConfigNode> {
        let selected_i = self.group_items_state.state.selected()?;
        Some(self.group_items.get(selected_i)?)
    }

    pub fn update_description(&mut self) -> Option<()> {
        let selected_node = self.get_selected_node()?;
        self.description = &selected_node.description;

        Some(())
    }

    // TODO methods move down and move up maintain the breadcrumb stack
    pub fn enter_selected_node(&mut self) -> Option<()> {
        let selected_node = self.get_selected_node()?;

        if !selected_node.is_leaf() {
            // self.set_config_for_node(selected_node);
            self.breadcrumbs.push(selected_node); // TODO this clears breadcrumbs for some reason
        }

        Some(())
    }

    pub fn exit_current_node(&mut self) -> Option<()> {
        if self.breadcrumbs.len() > 1 {
            let previous_node = self.breadcrumbs.pop()?;
            self.set_config_for_node(previous_node);
        }

        Some(())
    }
}


impl<'a> fmt::Debug for UiState<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UiState")
            .field("current_title", &self.title)
            .field("current_description", &self.description)
            .field("current_breadcrumbs", &self.breadcrumbs)
            .field("current_group_items", &self.group_items)
            .field("current_command_output", &self.command_output)
            .field("current_selected", &self.group_items_state.state.selected())
            .field("current_state_list", &self.group_items_state.items)
            .finish()
    }
}
