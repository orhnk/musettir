mod models;
mod messages;
mod widgets;
mod ui;

use ui::Win;

fn main() {
    relm::run::<Win>(()).unwrap();
}