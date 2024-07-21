use gtk::{glib, Application, ApplicationWindow, Grid};
use gtk::{prelude::*, Button};

mod emoji_ls;

const APP_ID: &str = "org.sparklet.windot";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let grid = Grid::builder()
        .column_spacing(10)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .row_homogeneous(true)
        .column_homogeneous(true)
        .build();

    for i in 0..10 {
        let button = Button::builder().label(i.to_string()).build();

        button.connect_clicked(|button| {
            button.set_label("Hello World!");
        });

        grid.attach(&button, i, 0, 1, 1);
    }

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Select an emoji.")
        .child(&grid)
        .build();

    // Present window
    window.present();
}
