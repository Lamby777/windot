use std::sync::OnceLock;

use emojis::{Emoji, Group};
use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Button, CssProvider, Grid,
    Orientation, ScrolledWindow, SearchEntry, Stack, StackSidebar,
};

mod components;
use components::*;

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

static WINDOW: OnceLock<SApplicationWindow> = OnceLock::new();

/// Wrapper around `ApplicationWindow` to implement `Send` and `Sync`.
#[derive(Debug)]
struct SApplicationWindow(ApplicationWindow);
unsafe impl Sync for SApplicationWindow {}
unsafe impl Send for SApplicationWindow {}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.connect_startup(|_| load_css());
    app.run()
}

fn load_css() {
    let css = CssProvider::new();
    css.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
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
    let search_pane = {
        let search = build_search();
        let name = "🔎 Search";
        stack.add_titled(&search, Some(&name), &name);
        search
    };

    // build the "all" stack
    {
        let grid = build_grid(all_emojis());
        let name = "🌍 All";
        stack.add_titled(&grid, Some(&name), &name);
    }

    // build the group stacks
    for group in GROUPS {
        let grid = build_grid(all_emojis().filter(|e| e.group() == *group));
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

    search_pane.first_child().unwrap().grab_focus();

    // Present window
    window.present();
    WINDOW.set(SApplicationWindow(window)).unwrap();
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
