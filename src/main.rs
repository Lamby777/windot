use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::RwLock;

use adw::Application;
use emojis::{Emoji, SkinTone};
use gtk::prelude::*;
use gtk::{
    glib, ApplicationWindow, Button, CssProvider, Grid, Orientation,
    ScrolledWindow, SearchEntry, Stack, StackSidebar,
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
    app.connect_activate(build_window);
    app.connect_startup(|_| load_css());
    app.run()
}

fn on_emoji_picked(button: &Button, window: &ApplicationWindow) {
    let emoji = button.label().unwrap();
    println!("Picked: {}", emoji);
    cli_clipboard::set_contents(emoji.to_string()).unwrap();

    {
        let mut conf = CONFIG.write().unwrap();
        let conf = conf.as_mut().unwrap();

        // push to recents
        let emoji = every_emoji_and_variants().find(|e| **e == *emoji).unwrap();
        if !conf.recent_emojis.contains(&emoji) {
            conf.recent_emojis.push(emoji);
        }
    }

    println!("Closing...");
    window.close();
}

fn on_variants_request(button: &Button, window: &Rc<ApplicationWindow>) {
    let emoji = button.label().unwrap();
    println!("Requesting Variants: {}", emoji);

    let emoji = every_emoji_and_variants().find(|e| **e == *emoji).unwrap();
    let Some(skin_tones_iter) = emoji.skin_tones() else {
        println!("No variants, returning.");
        return;
    };

    let variant_grid = build_grid(window.clone(), skin_tones_iter);
    let stack: Stack = window
        .child()
        .unwrap()
        .last_child()
        .unwrap()
        .downcast()
        .unwrap();

    let variants = stack
        .child_by_name("Variants")
        .unwrap()
        .downcast::<gtk::Box>()
        .unwrap();

    variants.append(&variant_grid);

    stack.set_visible_child_name("Variants");
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

fn build_window(app: &Application) {
    // Create a window
    let window = Rc::new(
        ApplicationWindow::builder()
            .application(app)
            .title("Select an emoji.")
            .build(),
    );

    // Present window
    let main_box = build_main_box(&window);
    window.set_child(Some(&main_box));
    window.present();

    window.connect_close_request(|_| {
        CONFIG.read().unwrap().as_ref().unwrap().save();

        glib::Propagation::Proceed
    });
}

fn all_emojis_in_preferred_tone() -> impl Iterator<Item = &'static Emoji> {
    let preferred_tone =
        CONFIG.read().unwrap().as_ref().unwrap().preferred_skin_tone;

    emojis::iter().map(move |e| e.with_skin_tone(preferred_tone).unwrap_or(e))
}

fn every_emoji_and_variants() -> impl Iterator<Item = &'static Emoji> {
    emojis::iter().flat_map(|e| {
        // skin_tones returns None if there are no skin tones, so we need to
        // return the emoji itself in that case. BUT skin_tones also contains
        // the default emoji skin tone as well, so we can't just chain it on
        let tones = e.skin_tones();
        let default = std::iter::once(e);

        let mut tones_only = tones.into_iter().flatten();
        tones_only.next(); // skip the default skin tone

        default.chain(tones_only)
    })
}
