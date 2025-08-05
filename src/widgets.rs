use gtk::{Window, Button, Box as GtkBox, DrawingArea, ScrolledWindow, ColorButton};

pub struct Widgets {
    pub window: Window,
    pub open_button: Button,
    pub color_button: ColorButton,
    pub drawing_area: DrawingArea,
    pub scrolled_window: ScrolledWindow,
    pub main_box: GtkBox,
    pub button_box: GtkBox,
}
