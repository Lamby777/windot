use super::*;

use std::time::Duration;

use glib::source::timeout_add_local;
use glib::ControlFlow;
use gtk::{Align, FlowBox, Separator, Viewport};

/// Includes all the single-person skin tones.
///
/// For multiple people, the default skin tone is shown and the variant
/// should be picked manually.
const PREFERRABLE_SKIN_TONES: &[SkinTone] = &[
    SkinTone::Default,
    SkinTone::Light,
    SkinTone::MediumLight,
    SkinTone::Medium,
    SkinTone::MediumDark,
    SkinTone::Dark,
];

/// Run this after changing any config values.
/// It will rebuild the whole UI again and show it with the new settings in mind.
pub fn reapply_main_box(window: &ApplicationWindow, focus_search: bool) {
    let (main_box, search_box) = build_main_box(window);
    window.set_child(Some(&main_box));

    if focus_search {
        search_box.grab_focus();
    }
}

pub fn build_main_box(
    window: &ApplicationWindow,
) -> (gtk::Box, gtk::SearchEntry) {
    let main_box = gtk::Box::builder()
        .spacing(10)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .orientation(Orientation::Horizontal)
        .build();

    let stack = Stack::builder()
        .height_request(400)
        .vhomogeneous(false)
        .build();
    let sidebar = StackSidebar::builder()
        .width_request(200)
        .height_request(500)
        .stack(&stack)
        .build();

    // build the "search" stack
    let search_box = {
        let search_pane = build_search(window);
        let name = "🔎 Search";
        stack.add_titled(&search_pane, Some(name), name);

        search_pane
            .first_child()
            .unwrap()
            .downcast::<SearchEntry>()
            .unwrap()
    };

    let window2 = window.clone();
    let key_controller = gtk::EventControllerKey::new();
    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        if keyval == Key::Escape {
            window2.set_visible(false);
        }

        glib::Propagation::Proceed
    });
    search_box.add_controller(key_controller);

    // build the "recents" stack
    {
        let search = build_grid(
            window,
            CONFIG
                .read()
                .unwrap()
                .as_ref()
                .unwrap()
                .recent_emojis
                .clone()
                .into_iter(),
        );
        let name = "🕒 Recents";
        stack.add_titled(&search, Some(name), name);
    };

    // the invisible "variants" stack
    {
        let variant_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        let label = gtk::Label::builder()
            .label("Variants")
            .name("variants-title")
            .build();
        variant_box.append(&label);

        stack.add_named(&variant_box, Some("Variants"));
    }

    // build the group stacks
    for group in GROUPS {
        let grid = build_grid(
            window,
            all_emojis_in_preferred_tone().filter(|e| e.group() == *group),
        );
        let name = group_display_name(*group);
        stack.add_titled(&grid, Some(name), name);
    }

    // build the "settings" stack
    {
        let search = build_settings(window);
        let name = "⚙️ Settings";
        stack.add_titled(&search, Some(name), name);
        search
    };

    main_box.append(&sidebar);
    main_box.append(&stack);

    // search_pane.first_child().unwrap()
    (main_box, search_box)
}

pub fn build_settings(window: &ApplicationWindow) -> gtk::Box {
    let stack = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let settings_label = gtk::Label::builder()
        .label("Settings")
        .name("settings-title")
        .build();

    let sep = Separator::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(5)
        .margin_bottom(10)
        .build();

    let skin_tones_setting_box = gtk::Box::builder().hexpand(true).build();
    let skin_tones_box_label = gtk::Label::builder()
        .label("Preferred Skin Tone")
        .name("preferred-skin-tone")
        .build();
    let skin_tones_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .hexpand(true)
        .halign(Align::End)
        .build();

    for tone in PREFERRABLE_SKIN_TONES {
        let emoji = emojis::get("👋").unwrap().with_skin_tone(*tone).unwrap();
        let btn = Button::builder().label(emoji.to_string()).build();

        let window2 = window.clone();
        btn.connect_clicked(move |_| {
            {
                let mut conf = CONFIG.write().unwrap();
                let conf = conf.as_mut().unwrap();
                conf.preferred_skin_tone = *tone;
            }

            reapply_main_box(&window2, true);
        });

        skin_tones_box.append(&btn);
    }

    skin_tones_setting_box.append(&skin_tones_box_label);
    skin_tones_setting_box.append(&skin_tones_box);

    let clear_box = gtk::Box::builder().hexpand(true).build();
    let clear_label = gtk::Label::builder()
        .label("Clear Recent Emojis")
        .name("clear-recents")
        .build();
    let clear_btn = Button::builder()
        .label("CLEAR")
        .halign(Align::End)
        .hexpand(true)
        .build();

    clear_box.append(&clear_label);
    clear_box.append(&clear_btn);

    let window2 = window.clone();
    clear_btn.connect_clicked(move |_| {
        {
            let mut conf = CONFIG.write().unwrap();
            let conf = conf.as_mut().unwrap();
            conf.recent_emojis.clear();
        }
        reapply_main_box(&window2, true);
    });

    let settings_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .build();

    settings_box.append(&skin_tones_setting_box);
    settings_box.append(&clear_box);

    stack.append(&settings_label);
    stack.append(&sep);
    stack.append(&settings_box);

    stack
}

pub fn build_search(window: &ApplicationWindow) -> gtk::Box {
    let stack = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let searchbox = SearchEntry::builder().build();

    // Build the grid only once
    let flowbox = build_grid(window, all_emojis_in_preferred_tone());

    stack.append(&searchbox);
    stack.append(&flowbox);

    searchbox.connect_search_changed(move |sb| {
        // Reduced debounce time since operation is faster
        let debounce_time = Duration::from_millis(300);

        let search_text = sb.text().to_string().to_lowercase();
        let flowbox2 = flowbox.clone();

        timeout_add_local(debounce_time, move || {
            update_emoji_visibility(&flowbox2, &search_text);
            ControlFlow::Break
        });
    });

    stack
}

fn update_emoji_visibility(
    scrolled_window: &ScrolledWindow,
    search_text: &str,
) {
    // First get the Viewport
    if let Some(viewport) = scrolled_window
        .child()
        .and_then(|child| child.downcast::<Viewport>().ok())
    {
        // Then get the FlowBox from the Viewport
        if let Some(flowbox) = viewport
            .child()
            .and_then(|child| child.downcast::<FlowBox>().ok())
        {
            let n_items = flowbox.observe_children().n_items();

            for i in 0..n_items {
                let Some(child) = flowbox.child_at_index(i.try_into().unwrap())
                else {
                    continue;
                };

                if let Some(button) = child.first_child() {
                    if let Ok(button) = button.downcast::<Button>() {
                        // Get emoji data from button
                        if let Some(tooltip) = button.tooltip_text() {
                            let is_match = if search_text.is_empty() {
                                true
                            } else {
                                tooltip.to_lowercase().contains(search_text) ||
                                    // Optionally check shortcode if available
                                    button.label().and_then(|label| {
                                        emojis::get(&label).and_then(|emoji| {
                                            emoji.shortcode().map(|shortcode|
                                                shortcode.contains(search_text))
                                        })
                                    }).unwrap_or(false)
                            };

                            child.set_visible(is_match);
                        }
                    }
                }
            }
        }
    }
}

pub fn build_grid(
    window: &ApplicationWindow,
    emojis: impl Iterator<Item = &'static Emoji>,
) -> ScrolledWindow {
    let flowbox = FlowBox::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .row_spacing(10)
        .column_spacing(10)
        .homogeneous(true)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Start)
        .hexpand(true)
        .vexpand(true)
        .selection_mode(gtk::SelectionMode::None)
        .min_children_per_line(1)
        .max_children_per_line(100)
        .build();

    // Add all emojis to the FlowBox once
    for emoji in emojis {
        let button = make_button(emoji, window);
        button.set_hexpand(true);
        button.set_vexpand(true);
        flowbox.insert(&button, -1);
    }

    ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .hexpand(true)
        .vexpand(true)
        .propagate_natural_height(true)
        .propagate_natural_width(true)
        .child(&flowbox)
        .build()
}

fn make_button(emoji: &'static Emoji, window: &ApplicationWindow) -> Button {
    let button = Button::builder()
        .label(emoji.to_string())
        .height_request(36) // You can adjust this value
        // Optionally set width request if needed
        .width_request(36) // You can adjust this value
        .tooltip_text(emoji.name())
        // Make sure content stays centered
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();

    let window2 = window.clone();
    button.connect_clicked(move |b| on_emoji_picked(b, &window2));

    // if right click, show variants
    let gesture = gtk::GestureClick::new();
    gesture.set_button(gtk::gdk::ffi::GDK_BUTTON_SECONDARY as u32);

    let button2 = button.clone();
    let window2 = window.clone();
    gesture.connect_pressed(move |gesture, _, _, _| {
        gesture.set_state(gtk::EventSequenceState::Claimed);
        on_variants_request(&button2, &window2);
    });
    button.add_controller(gesture);

    button
}
