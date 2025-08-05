use gtk::{Window, Button, Box as GtkBox, DrawingArea, ScrolledWindow};

pub struct Widgets {
    pub window: Window,
    pub open_button: Button,
    pub drawing_area: DrawingArea,
    pub scrolled_window: ScrolledWindow,
    pub main_box: GtkBox,
}
