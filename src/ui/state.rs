use core::fmt;

use crate::config_reader::config_structs::ConfigNode;
use crate::ui::util::StatefulList;

pub struct UiState<'a> {
    pub current_title: &'a str,
    pub current_description: &'a str,
    pub current_breadcrumbs: Vec<&'a ConfigNode>,
    pub current_group_items: Vec<&'a ConfigNode>,
    pub current_group_items_state: StatefulList<&'a str>,
    pub current_command_output: String,
}

impl<'a> UiState<'a> {
    pub fn new() -> UiState<'a> {
        UiState {
            current_title: "",
            current_description: "",
            current_breadcrumbs: vec![],
            current_group_items: vec![],
            current_group_items_state: StatefulList::new(),
            current_command_output: "".to_string(),
        }
    }

    pub fn set_config_for_node(&mut self, node: &'a ConfigNode) {
        self.current_title = &node.name;
        self.current_description = &node.description;

        self.current_breadcrumbs = vec![];

        if let Some(children) = &node.children {
            self.current_group_items = children.iter().collect();


            let mut items_for_state = children
                .iter()
                .map(|e| e.name.as_str())
                .collect::<Vec<&str>>();

            // sort elements of group alphabetically to make it nicer
            items_for_state.sort_by_key(|e| e.to_lowercase());

            self.current_group_items_state = StatefulList::with_items(items_for_state);
        } else {
            self.current_group_items = vec![];
            self.current_group_items_state = StatefulList::new();
        }

        self.current_command_output = "potato".to_string();

        // finally, always set the first element of the state as selected
        self.current_group_items_state.next();
    }

    fn get_selected_node(&self) -> Option<&'a ConfigNode> {
        let selected_i = self.current_group_items_state.state.selected()?;
        Some(self.current_group_items.get(selected_i)?)
    }

    pub fn update_description(&mut self) -> Option<()> {
        let selected_node = self.get_selected_node()?;
        self.current_description = &selected_node.description;

        Some(())
    }

    // TODO methods move down and move up maintain the breadcrumb stack
    pub fn enter_selected_node(&mut self) -> Option<()> {
        let selected_node = self.get_selected_node()?;

        if !selected_node.is_leaf() {
            self.set_config_for_node(selected_node);
            self.current_breadcrumbs.push(selected_node); // TODO this clears breadcrumbs for some reason
        }

        Some(())
    }

    pub fn exit_current_node(&mut self) -> Option<()> {
        if self.current_breadcrumbs.len() > 1 {
            let previous_node = self.current_breadcrumbs.pop()?;
            self.set_config_for_node(previous_node);
        }

        Some(())
    }
}


impl<'a> fmt::Debug for UiState<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UiState")
            .field("current_title", &self.current_title)
            .field("current_description", &self.current_description)
            .field("current_breadcrumbs", &self.current_breadcrumbs)
            .field("current_group_items", &self.current_group_items)
            .field("current_command_output", &self.current_command_output)
            .field("current_selected", &self.current_group_items_state.state.selected())
            .field("current_state_list", &self.current_group_items_state.items)
            .finish()
    }
}
