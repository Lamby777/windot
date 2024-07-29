use std::rc::Rc;

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

pub fn build_settings() -> gtk::Box {
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
        let button = Button::builder().label(emoji.to_string()).build();

        button.connect_clicked(|_| {
            let mut conf = CONFIG.write().unwrap();
            let conf = conf.as_mut().unwrap();
            conf.preferred_skin_tone = *tone;
            conf.save();
        });

        skin_tones_box.append(&button);
    }

    skin_tones_setting_box.append(&skin_tones_box_label);
    skin_tones_setting_box.append(&skin_tones_box);

    stack.append(&settings_label);
    stack.append(&sep);
    stack.append(&skin_tones_setting_box);

    stack
}

pub fn build_search(window: Rc<ApplicationWindow>) -> gtk::Box {
    let stack = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let grid = build_grid(window.clone(), all_emojis_in_preferred_tone());

    let searchbox = SearchEntry::builder().build();

    stack.append(&searchbox);
    stack.append(&grid);

    searchbox.connect_search_changed(move |sb| {
        let parent = sb.parent().unwrap().downcast::<gtk::Box>().unwrap();
        parent.remove(&parent.last_child().unwrap());

        parent.append(&build_grid(
            window.clone(),
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
    window: Rc<ApplicationWindow>,
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
        on_variants_request(&button2, &window2)
    });
    button.add_controller(gesture);

    button
}
