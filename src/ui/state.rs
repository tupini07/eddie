use crate::config_reader::config_structs::ConfigNode;
use crate::ui::util::StatefulList;

#[derive(Debug)]
pub struct UiState<'a> {
    pub current_title: String,
    pub current_breadcrumbs: Vec<String>,
    pub current_group_items: StatefulList<&'a ConfigNode>,
    pub current_command_output: String,
}

impl<'a> UiState<'a> {
    pub fn new() -> UiState<'a> {
        UiState {
            current_title: "".to_string(),
            current_breadcrumbs: vec![],
            current_group_items: StatefulList::new(),
            current_command_output: "".to_string(),
        }
    }

    pub fn set_config_for_node(&mut self, node: &'a ConfigNode) {
        self.current_title = node.name.clone();
        self.current_breadcrumbs = vec![];

        if let Some(children) = &node.children {
            self.current_group_items = StatefulList::with_items(children
                .iter()
                .collect());
        } else {
            self.current_group_items = StatefulList::new()
        }

        self.current_command_output = "potato".to_string();
    }
}