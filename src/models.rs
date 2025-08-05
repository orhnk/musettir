use std::path::PathBuf;
use gtk::gdk_pixbuf::Pixbuf;

#[derive(Clone)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone)]
pub struct Model {
    pub current_image_path: Option<PathBuf>,
    pub current_pixbuf: Option<Pixbuf>,
    pub rectangles: Vec<Rectangle>,
    pub drawing_rectangle: Option<Rectangle>,
    pub is_drawing: bool,
}

impl Model {
    pub fn new() -> Self {
        Model {
            current_image_path: None,
            current_pixbuf: None,
            rectangles: Vec::new(),
            drawing_rectangle: None,
            is_drawing: false,
        }
    }

    pub fn set_image_path(&mut self, path: PathBuf) {
        self.current_image_path = Some(path);
    }

    pub fn set_pixbuf(&mut self, pixbuf: Pixbuf) {
        self.current_pixbuf = Some(pixbuf);
    }

    pub fn start_drawing(&mut self, x: f64, y: f64) {
        self.is_drawing = true;
        self.drawing_rectangle = Some(Rectangle { x, y, width: 0.0, height: 0.0 });
    }

    pub fn update_drawing(&mut self, x: f64, y: f64) {
        if let Some(ref mut rect) = self.drawing_rectangle {
            rect.width = x - rect.x;
            rect.height = y - rect.y;
        }
    }

    pub fn finish_drawing(&mut self) {
        if let Some(rect) = self.drawing_rectangle.take() {
            if rect.width.abs() > 5.0 && rect.height.abs() > 5.0 {
                self.rectangles.push(rect);
            }
        }
        self.is_drawing = false;
    }
}
