use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Align, Separator};

use super::*;

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
pub fn reapply_main_box(window: &Rc<ApplicationWindow>) {
    let main_box = build_main_box(&window);
    window.set_child(Some(&main_box));
}

pub fn build_main_box(window: &Rc<ApplicationWindow>) -> gtk::Box {
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
    let search_pane = {
        let search = build_search(window.clone());
        let name = "🔎 Search";
        stack.add_titled(&search, Some(name), name);
        search
    };

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

    search_pane.first_child().unwrap().grab_focus();

    main_box
}

pub fn build_settings(window: &Rc<ApplicationWindow>) -> gtk::Box {
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

            reapply_main_box(&window2);
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
        reapply_main_box(&window2);
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

pub fn build_search(window: Rc<ApplicationWindow>) -> gtk::Box {
    let stack = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let grid = build_grid(&window, all_emojis_in_preferred_tone());

    let searchbox = SearchEntry::builder().build();

    stack.append(&searchbox);
    stack.append(&grid);

    searchbox.connect_search_changed(move |sb| {
        let parent = sb.parent().unwrap().downcast::<gtk::Box>().unwrap();
        parent.remove(&parent.last_child().unwrap());

        parent.append(&build_grid(
            &window,
            all_emojis_in_preferred_tone().filter(|e| {
                e.with_skin_tone(SkinTone::Default)
                    .unwrap_or(e)
                    .shortcodes()
                    .any(|sc| sc.contains(&sb.text().to_string()))
            }),
        ));
    });

    stack
}

pub fn build_grid(
    window: &Rc<ApplicationWindow>,
    emojis: impl Iterator<Item = &'static Emoji>,
) -> ScrolledWindow {
    let grid = Grid::builder()
        .column_spacing(10)
        .row_homogeneous(true)
        .column_homogeneous(true)
        .build();

    let mut row = 0;
    let mut col = 0;

    for emoji in emojis {
        let button = make_button(emoji, window.clone());
        grid.attach(&*button, col, row, 1, 1);

        col += 1;

        if col == EMOJIS_PER_ROW {
            col = 0;
            row += 1;
        }
    }

    ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .width_request(500)
        // .height_request(400)
        .vexpand(true)
        .child(&grid)
        .build()
}

fn make_button(
    emoji: &'static Emoji,
    window: Rc<ApplicationWindow>,
) -> Rc<Button> {
    let button = Rc::new(Button::builder().label(emoji.to_string()).build());

    let window2 = window.clone();
    button.connect_clicked(move |b| on_emoji_picked(b, &window));

    // if right click, show variants
    let gesture = gtk::GestureClick::new();
    gesture.set_button(gtk::gdk::ffi::GDK_BUTTON_SECONDARY as u32);

    let button2 = button.clone();
    gesture.connect_pressed(move |gesture, _, _, _| {
        gesture.set_state(gtk::EventSequenceState::Claimed);
        on_variants_request(&button2, &window2);
    });
    button.add_controller(gesture);

    button
}
