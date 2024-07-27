use std::rc::Rc;

use super::*;

pub fn build_search(window: Rc<ApplicationWindow>) -> gtk::Box {
    let stack = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let grid = build_grid(window.clone(), all_emojis());

    let searchbox = SearchEntry::builder().build();

    stack.append(&searchbox);
    stack.append(&grid);

    searchbox.connect_search_changed(move |sb| {
        let parent = sb.parent().unwrap().downcast::<gtk::Box>().unwrap();
        parent.remove(&parent.last_child().unwrap());

        parent.append(&build_grid(
            window.clone(),
            all_emojis().filter(|e| {
                e.shortcodes().any(|sc| sc.contains(&sb.text().to_string()))
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
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
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
