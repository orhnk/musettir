mod models;
mod messages;
mod widgets;
mod ui;

use ui::Win;

fn main() {
    gtk::init().expect("Failed to initialize GTK");
    relm::run::<Win>(()).unwrap();
}