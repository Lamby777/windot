use emojis::{Emoji, Group};
use gtk::{
    glib, Application, ApplicationWindow, Button, Grid, Orientation, SearchEntry, Stack,
    StackSidebar,
};
use gtk::{prelude::*, ScrolledWindow};

const APP_ID: &str = "org.sparklet.windot";
const EMOJIS_PER_ROW: i32 = 10;

const GROUPS: &[Group] = &[
    Group::SmileysAndEmotion,
    Group::PeopleAndBody,
    Group::AnimalsAndNature,
    Group::Activities,
    Group::FoodAndDrink,
    Group::Objects,
    Group::TravelAndPlaces,
    Group::Symbols,
    Group::Flags,
];

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
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
        .stack(&stack)
        .build();

    // build the "search" stack
    {
        let search = build_search();
        let name = "🔎 Search";
        stack.add_titled(&search, Some(&name), &name);
    }

    // build the "all" stack
    {
        let grid = build_grid(all_emojis());
        let name = "🌍 All";
        stack.add_titled(&grid, Some(&name), &name);
    }

    // build the group stacks
    for group in GROUPS {
        let grid = build_grid(group.emojis());
        let name = group_display_name(*group);
        stack.add_titled(&grid, Some(&name), &name);
    }

    main_box.append(&sidebar);
    main_box.append(&stack);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Select an emoji.")
        .child(&main_box)
        .build();

    // Present window
    window.present();
}

fn build_search() -> gtk::Box {
    let stack = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let grid = build_grid(all_emojis());

    let searchbox = SearchEntry::builder().build();

    stack.append(&searchbox);
    stack.append(&grid);

    searchbox.connect_search_changed(move |sb| {
        let parent: gtk::Box = unsafe { sb.parent().unwrap().unsafe_cast() };
        parent.remove(&parent.last_child().unwrap());

        parent.append(&build_grid(all_emojis().filter(|e| {
            e.shortcodes().any(|sc| sc.contains(&sb.text().to_string()))
        })));
    });

    stack
}

fn build_grid(emojis: impl Iterator<Item = &'static Emoji>) -> ScrolledWindow {
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

    // for emoji in emojis.take(EMOJIS_PER_PAGE) {
    for emoji in emojis {
        let button = Button::builder().label(emoji.to_string()).build();

        button.connect_clicked(|button| {
            let emoji = button.label().unwrap();
            println!("Button clicked: {}", emoji);
            cli_clipboard::set_contents(emoji.to_string()).unwrap();
        });
        grid.attach(&button, col, row, 1, 1);

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
        .height_request(400)
        .child(&grid)
        .build()
}

// getter in case i gotta change this later
fn all_emojis() -> impl Iterator<Item = &'static Emoji> {
    emojis::iter()
}

fn group_display_name(group: Group) -> &'static str {
    match group {
        Group::SmileysAndEmotion => "😄 Smileys & Emotion",
        Group::PeopleAndBody => "🧑 People & Body",
        Group::AnimalsAndNature => "🐷 Animals & Nature",
        Group::Activities => "⚽ Activities",
        Group::FoodAndDrink => "🍕 Food & Drink",
        Group::Objects => "🧦 Objects",
        Group::TravelAndPlaces => "✈️ Travel & Places",
        Group::Symbols => "☢️ Symbols",
        Group::Flags => "🏳️‍⚧️ Flags",
    }
}
