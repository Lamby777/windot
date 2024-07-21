use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, Button};

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

    for i in 1..=10 {
        // TODO seriously? can't this go before the loop?
        // This is kinda ridiculous.
        let text = i.to_string();
        let builder = Button::builder().label(text);
        let button = builder.build();

        button.connect_clicked(move |_| {
            eprintln!("Clicked! {}", i);
        });

        res.push(button);
    }

    res
}
