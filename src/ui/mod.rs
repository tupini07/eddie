use std::rc::Rc;

use cursive::align::{Align, HAlign};
use cursive::event::Key;
use cursive::traits::*;
use cursive::view::IntoBoxedView;
use cursive::views::{Dialog, LinearLayout, Panel, SelectView, TextArea, TextView};

use crate::config_reader::config_structs::{AppConfig, ConfigNode, EddieConfig};

fn is_group_node(node: &ConfigNode) -> bool {
    node.children.is_some()
}

fn get_description_for_node(node: &ConfigNode) -> String {
    let node_description = &node.description;
    let node_type = if is_group_node(node) {
        "Group"
    } else {
        "Command"
    };

    format!("[Type: {}] {}", node_type, node_description)
}

fn create_group_layer(
    eddie_config: Rc<EddieConfig>,
    node: Rc<ConfigNode>,
    breadcrumbs: Vec<String>,
) -> impl IntoBoxedView {
    let flat_bread: String = breadcrumbs
        .iter()
        .map(|e| e.clone())
        .collect::<Vec<String>>()
        .join(" / ");

    let breadcrumbs_text = TextView::new(format!("{} / {}", eddie_config.ship_name, flat_bread));
    let section_title = Dialog::around(TextView::new(&node.name));

    let mut group_items = SelectView::new()
        .h_align(HAlign::Left)
        .align(Align::top_left())
        .autojump();

    let mut first_description_text = String::new();

    if let Some(child_nodes) = &node.children {
        let mut is_set = false;
        for child_node in child_nodes {
            if !is_set {
                first_description_text = get_description_for_node(&child_node);
            }
            is_set = true;

            group_items.add_item(&child_node.name, Rc::clone(child_node));
        }
    } else {
        unreachable!("When rendering a group this should always be a non-terminal node!");
    }
    // let child_nodes = node
    //     .children
    //     .as_ref()
    //     .expect("When rendering a group this should always be a non-terminal node!");

    // for child_node in child_nodes {
    //     group_items.add_item(&child_node.name, child_node);
    // }

    let item_description_text = TextArea::new()
        .disabled()
        .content(first_description_text)
        // .content(get_description_for_node(&child_nodes.get(0).unwrap()))
        .with_name("item_description_text");

    group_items.set_on_select(|s, child_node| {
        s.call_on_name("item_description_text", |view: &mut TextArea| {
            view.set_content(get_description_for_node(child_node));
        });
    });

    group_items.set_on_submit(move |s, child_node: &Rc<ConfigNode>| {
        let mut new_breadcrumbs = breadcrumbs.clone();
        new_breadcrumbs.push(child_node.name.clone());

        s.add_fullscreen_layer(create_group_layer(
            Rc::clone(&eddie_config),
            Rc::clone(child_node),
            new_breadcrumbs,
        ));
    });

    let command_output = Panel::new(
        TextArea::new()
            .content("Write description here...")
            .disabled(),
    )
    .title("Command outputs")
    .title_position(HAlign::Left);

    let middle_layout = LinearLayout::horizontal()
        .child(
            Panel::new(group_items)
                .title("Group Items")
                .title_position(HAlign::Left)
                .full_width(),
        )
        .child(command_output.full_width())
        .full_width();

    let item_description = Panel::new(item_description_text)
        .title("Item description (Type: Group)")
        .title_position(HAlign::Left);

    let help_text = TextView::new("TAB to select next / Shift + TAB to select previous / RETURN to select / BACKSPACE to go back");

    let vertical_layout = LinearLayout::vertical()
        .child(breadcrumbs_text)
        .child(section_title)
        .child(middle_layout.full_height())
        .child(item_description)
        .child(help_text)
        .full_screen();

    Dialog::around(vertical_layout)
}

pub fn show_ui(app_config: AppConfig) {
    let mut siv = cursive::default();

    siv.set_window_title("🚀 eddie");

    let eddie_config_rc = app_config.eddie_config;
    let root_node_rc = app_config.config_tree;

    let root_layer = create_group_layer(eddie_config_rc, root_node_rc, vec![]);

    siv.add_fullscreen_layer(root_layer);

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(Key::Backspace, |s| {
        // only pop screens if we're not at the very first screen
        if s.screen().len() > 1 {
            // this doesn't work
            s.pop_layer();
        }
    });

    siv.run();
}
