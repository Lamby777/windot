use gtk::{glib, Application, ApplicationWindow, Grid};
use gtk::{prelude::*, Button};

mod emoji_ls;

const APP_ID: &str = "org.sparklet.windot";
const EMOJIS_PER_ROW: i32 = 10;

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

    // just piggies for now
    let emojis = ['🐷'; 21];

    let mut row = 0;
    let mut col = 0;

    for emoji in emojis.iter() {
        let button = Button::builder().label(emoji.to_string()).build();

        button.connect_clicked(|button| {
            println!("Button clicked: {}", button.label().unwrap());
        });
        grid.attach(&button, col, row, 1, 1);

        col += 1;

        if col == EMOJIS_PER_ROW {
            col = 0;
            row += 1;
        }
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
