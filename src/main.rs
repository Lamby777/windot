use std::fs;
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};

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

static WINDOW: OnceLock<SApplicationWindow> = OnceLock::new();
static CONFIG: RwLock<Option<Config>> = RwLock::new(None);

/// Wrapper around `ApplicationWindow` to implement `Send` and `Sync`.
#[derive(Debug)]
struct SApplicationWindow(ApplicationWindow);
unsafe impl Sync for SApplicationWindow {}
unsafe impl Send for SApplicationWindow {}

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

fn on_emoji_picked(button: &Button, close: bool) {
    let emoji = button.label().unwrap();
    println!("Picked: {}", emoji);

    cli_clipboard::set_contents(emoji.to_string()).unwrap();
    CONFIG
        .write()
        .unwrap()
        .as_mut()
        .unwrap()
        .recent_emojis
        .push(emojis::iter().find(|e| **e == *emoji).unwrap());

    CONFIG.read().unwrap().as_ref().unwrap().save();

    if !close {
        return;
    }

    println!("Closing...");
    WINDOW.get().unwrap().0.close();
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

    // build the "search" stack
    {
        let search = build_grid(
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
