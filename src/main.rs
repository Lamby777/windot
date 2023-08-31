use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, Button};
use gtk4 as gtk;

mod emoji_ls;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.sparklet.windot")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(440)
            .default_height(440)
            .title("Hello, World!")
            .build();

        let buttons: Vec<_> = make_emoji_buttons();

        for b in buttons.iter() {
            window.set_child(Some(b));
        }

        // Show the window.
        window.present();
    });

    app.run()
}

fn make_emoji_buttons() -> Vec<Button> {
    let mut res = vec![];

    for _ in 0..10 {
        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });

        res.push(button);
    }

    res
}
