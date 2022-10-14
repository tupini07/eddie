use std::rc::Rc;

use cursive::align::{Align, HAlign};
use cursive::event::Key;
use cursive::traits::*;
use cursive::utils::span::IndexedSpanRefMut;
use cursive::view::IntoBoxedView;
use cursive::views::{Dialog, LinearLayout, NamedView, Panel, SelectView, TextArea, TextView};
use tui::text::Text;

use crate::config_reader::config_structs::{AppConfig, ConfigNode, EddieConfig};

mod executor;

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

fn get_breadcrumbs(
    eddie_config: &EddieConfig,
    node: &ConfigNode,
    current_crumbs: &Vec<String>,
) -> TextView {
    let flat_bread: String = current_crumbs
        .iter()
        .map(|e| e.clone())
        .collect::<Vec<String>>()
        .join(" / ");

    TextView::new(format!("{} / {}", eddie_config.ship_name, flat_bread))
}

fn get_section_title(node: &ConfigNode) -> Dialog {
    Dialog::around(TextView::new(&node.name))
}

fn get_group_items(node: &ConfigNode) -> SelectView<Rc<ConfigNode>> {
    let mut group_items = SelectView::new()
        .h_align(HAlign::Left)
        .align(Align::top_left())
        .autojump();

    if let Some(child_nodes) = &node.children {
        for child_node in child_nodes {
            group_items.add_item(&child_node.name, Rc::clone(child_node));
        }
    } else {
        unreachable!("When rendering a group this should always be a non-terminal node!");
    }

    group_items
}

fn get_item_description(
    node: &ConfigNode,
    group_items: &SelectView<Rc<ConfigNode>>,
) -> (String, Panel<NamedView<TextArea>>) {
    let description_view_name = format!("item_description_text-{}", node.name);

    let mut first_description_text = group_items
        .get_item(0) // we know there is at least 1 item in the group, so this is safe to do
        .unwrap()
        .1
        .description
        .clone();

    let item_text = TextArea::new()
        .disabled()
        .content(first_description_text)
        .with_name(&description_view_name);

    let panel = Panel::new(item_text)
        .title("Item description (Type: Group)")
        .title_position(HAlign::Left);

    (description_view_name, panel)
}

fn get_command_output(node: &ConfigNode) -> (String, Panel<NamedView<TextArea>>) {
    let command_output_text_name = format!("command_output_text-{}", node.name);
    let command_output_text = TextArea::new()
        .disabled()
        .content("")
        .with_name(&command_output_text_name);

    let command_output = Panel::new(command_output_text)
        .title("Command outputs")
        .title_position(HAlign::Left);

    (command_output_text_name, command_output)
}

fn create_group_layer(
    eddie_config: Rc<EddieConfig>,
    node: Rc<ConfigNode>,
    breadcrumbs: Vec<String>,
) -> impl IntoBoxedView {
    // set breadcrumbs and group title

    let breadcrumbs_text = get_breadcrumbs(&eddie_config, &node, &breadcrumbs);
    let section_title = get_section_title(&node);

    // construct list of items in group
    let mut group_items = get_group_items(&node);

    let (description_view_name, item_description) = get_item_description(&node, &group_items);

    // what to do when we change the selected item
    group_items.set_on_select(move |s, child_node| {
        s.call_on_name(&description_view_name, |view: &mut TextArea| {
            view.set_content(get_description_for_node(child_node));
        });
    });

    let (command_output_text_name, command_output) = get_command_output(&node);

    // what to do when we actually select an item
    group_items.set_on_submit(move |s, child_node: &Rc<ConfigNode>| {
        let is_selected_node_group = is_group_node(&child_node);

        if is_selected_node_group {
            // if the selected node is a group then create a new view for it
            let mut new_breadcrumbs = breadcrumbs.clone();
            new_breadcrumbs.push(child_node.name.clone());

            s.add_fullscreen_layer(create_group_layer(
                Rc::clone(&eddie_config),
                Rc::clone(child_node),
                new_breadcrumbs,
            ));
        } else {
            // otherwise it means the selected node is a command, in which case we
            // execute it
            let output = executor::execute_command(&eddie_config, child_node);
            s.call_on_name(&command_output_text_name, |view: &mut TextArea| {
                view.set_content(output);
            });
        }
    });

    let middle_layout = LinearLayout::horizontal()
        .child(
            Panel::new(group_items)
                .title("Group Items")
                .title_position(HAlign::Left)
                .full_width(),
        )
        .child(command_output.full_width())
        .full_width();

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

    siv.add_global_callback('h', |s| {
        let help_text = r#"
This is some long form help text

This should be embedded in the executable from an external
file, so that it is easier to maintain.

hopefully it is scrollable

press BACKSPACE to close this window

Do we display emojis? 🚀😅🤔🙂

----------

something else

Lorem Ipsum is simply dummy text of the printing
and typesetting industry. Lorem Ipsum has been 
the industry's standard dummy text ever since the 
1500s, when an unknown printer took a galley of 
type and scrambled it to make a type specimen 
book. It has survived not only five centuries, 
but also the leap into electronic typesetting, remaining 
essentially unchanged. It was popularised in the 
1960s with the release of Letraset sheets 
containing Lorem

Lorem Ipsum is simply dummy text of the printing
and typesetting industry. Lorem Ipsum has been 
the industry's standard dummy text ever since the 
1500s, when an unknown printer took a galley of 
type and scrambled it to make a type specimen 
book. It has survived not only five centuries, 
but also the leap into electronic typesetting, remaining 
essentially unchanged. It was popularised in the 
1960s with the release of Letraset sheets 
containing Lorem
        "#;

        s.add_layer(Dialog::around(TextView::new(help_text).scrollable()).title("Help"));
    });

    siv.add_global_callback(Key::Backspace, |s| {
        // only pop screens if we're not at the very first screen
        if s.screen().len() > 1 {
            // this doesn't work
            s.pop_layer();
        }
    });

    siv.run();
}
