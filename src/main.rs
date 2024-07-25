use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::RwLock;

use emojis::Emoji;
use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Button, CssProvider, Grid,
    Orientation, ScrolledWindow, SearchEntry, Stack, StackSidebar,
};

mod components;
mod config;
mod consts;

use components::*;
use config::*;
use consts::*;

static CONFIG: RwLock<Option<Config>> = RwLock::new(None);

fn main() -> glib::ExitCode {
    // make the user data folder
    let data_dir = user_data_dir();
    if !data_dir.exists() && fs::create_dir_all(&data_dir).is_err() {
        eprintln!("warning: could not create data directory.");
    }
    let config = Config::load_or_create();
    CONFIG.write().unwrap().replace(config);

    // start the app
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.connect_startup(|_| load_css());
    app.run()
}

fn on_emoji_picked(button: &Button, window: &ApplicationWindow) {
    let emoji = button.label().unwrap();
    println!("Picked: {}", emoji);
    cli_clipboard::set_contents(emoji.to_string()).unwrap();

    let mut conf = CONFIG.write().unwrap();
    let conf = conf.as_mut().unwrap();

    // push to recents
    let emoji = emojis::iter().find(|e| **e == *emoji).unwrap();
    if !conf.recent_emojis.contains(&emoji) {
        conf.recent_emojis.push(emoji);
    }

    println!("Closing...");
    conf.save();
    window.close();
}

fn on_variants_request(button: &Button, window: &Rc<ApplicationWindow>) {
    let emoji = button.label().unwrap();
    println!("Requesting Variants: {}", emoji);

    // push to recents
    let emoji = emojis::iter().find(|e| **e == *emoji).unwrap();
    let Some(skin_tones_iter) = emoji.skin_tones() else {
        println!("No variants, returning.");
        return;
    };

    // something here is causing a segfault...
    let variant_grid = build_grid(window.clone(), skin_tones_iter);
    let stack: Stack = window
        .child()
        .unwrap()
        .last_child()
        .unwrap()
        .downcast()
        .unwrap();
    let last_child = stack.last_child().unwrap();
    stack.remove(&last_child);

    stack.add_named(&variant_grid, Some("🔄 Variants"));
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
    // Create a window
    let window = Rc::new(
        ApplicationWindow::builder()
            .application(app)
            .title("Select an emoji.")
            .build(),
    );

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
        let search = build_search(window.clone());
        let name = "🔎 Search";
        stack.add_titled(&search, Some(&name), &name);
        search
    };

    // build the "search" stack
    {
        let search = build_grid(
            window.clone(),
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
        stack.add_titled(&search, Some(&name), &name);
    };

    // build the group stacks
    for group in GROUPS {
        let grid = build_grid(
            window.clone(),
            all_emojis().filter(|e| e.group() == *group),
        );
        let name = group_display_name(*group);
        stack.add_titled(&grid, Some(&name), &name);
    }

    main_box.append(&sidebar);
    main_box.append(&stack);

    search_pane.first_child().unwrap().grab_focus();

    // Present window
    window.set_child(Some(&main_box));
    window.present();
}

// getter in case i gotta change this later
fn all_emojis() -> impl Iterator<Item = &'static Emoji> {
    emojis::iter()
}
