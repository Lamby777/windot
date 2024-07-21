use emojis::{Emoji, Group};
use gtk::{glib, Application, ApplicationWindow, Grid, Stack};
use gtk::{prelude::*, Button};

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
    let stack = Stack::new();

    for group in GROUPS {
        let grid = build_grid(*group);
        let name = format!("{:?}", group);
        stack.add_titled(&grid, Some(&name), &name);
    }

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Select an emoji.")
        .child(&stack)
        .build();

    // Present window
    window.present();
}

fn build_grid(group: Group) -> Grid {
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

    for emoji in group.emojis().take(10) {
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

    grid
}
