use relm::{Widget, Relm, Update, connect};
use gtk::prelude::*;
use gtk::{
    Button, Window, WindowType, Box as GtkBox, Orientation,
    Inhibit, Align, FileChooserDialog, FileChooserAction, ResponseType,
    DrawingArea, ScrolledWindow, PolicyType,
};
use gtk::gdk_pixbuf::{Pixbuf, InterpType};
use gtk::gdk::EventMask;
use gtk::cairo::Context;
use std::rc::Rc;
use std::cell::RefCell;

use crate::models::Model;
use crate::messages::Msg;
use crate::widgets::Widgets;

pub struct Win {
    pub model: Model,
    pub widgets: Widgets,
    pub model_for_draw: Rc<RefCell<Model>>,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model::new()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::OpenFile => {
                let dialog = FileChooserDialog::new(
                    Some("Open Image File"),
                    Some(&self.widgets.window),
                    FileChooserAction::Open,
                );
                
                // Add image file filters
                let filter = gtk::FileFilter::new();
                filter.add_mime_type("image/*");
                filter.set_name(Some("Image Files"));
                dialog.add_filter(&filter);
                
                dialog.add_button("Cancel", ResponseType::Cancel);
                dialog.add_button("Open", ResponseType::Accept);

                if dialog.run() == ResponseType::Accept {
                    if let Some(filename) = dialog.filename() {
                        self.model.set_image_path(filename.clone());
                        // Load the image
                        if let Some(pixbuf) = Pixbuf::from_file(&filename).ok() {
                            // Scale image to fit window while maintaining aspect ratio
                            let window_size = 800;
                            let scaled_pixbuf = if pixbuf.width() > window_size || pixbuf.height() > window_size {
                                let scale = (window_size as f64 / pixbuf.width().max(pixbuf.height()) as f64).min(1.0);
                                let new_width = (pixbuf.width() as f64 * scale) as i32;
                                let new_height = (pixbuf.height() as f64 * scale) as i32;
                                pixbuf.scale_simple(new_width, new_height, InterpType::Bilinear)
                            } else {
                                Some(pixbuf)
                            };
                            
                            if let Some(scaled) = scaled_pixbuf {
                                self.model.set_pixbuf(scaled.clone());
                                self.model.rectangles.clear();
                                
                                // Update the shared model for drawing
                                *self.model_for_draw.borrow_mut() = self.model.clone();
                                
                                self.widgets.drawing_area.set_size_request(scaled.width(), scaled.height());
                                self.widgets.drawing_area.queue_draw();
                            }
                        }
                    }
                }
                dialog.close();
            }
            Msg::LoadImage(path) => {
                if let Some(pixbuf) = Pixbuf::from_file(&path).ok() {
                    self.model.set_pixbuf(pixbuf);
                    *self.model_for_draw.borrow_mut() = self.model.clone();
                    self.widgets.drawing_area.queue_draw();
                }
            }
            Msg::StartDrawing(x, y) => {
                if self.model.current_pixbuf.is_some() {
                    self.model.start_drawing(x, y);
                    *self.model_for_draw.borrow_mut() = self.model.clone();
                }
            }
            Msg::UpdateDrawing(x, y) => {
                if self.model.is_drawing && self.model.current_pixbuf.is_some() {
                    self.model.update_drawing(x, y);
                    *self.model_for_draw.borrow_mut() = self.model.clone();
                    self.widgets.drawing_area.queue_draw();
                }
            }
            Msg::FinishDrawing => {
                self.model.finish_drawing();
                *self.model_for_draw.borrow_mut() = self.model.clone();
                self.widgets.drawing_area.queue_draw();
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("Image Viewer with Drawing");
        window.set_default_size(1000, 800);

        let main_box = GtkBox::new(Orientation::Vertical, 5);
        main_box.set_margin_start(10);
        main_box.set_margin_end(10);
        main_box.set_margin_top(10);
        main_box.set_margin_bottom(10);

        let open_button = Button::with_label("Open Image");
        open_button.set_halign(Align::Center);
        main_box.pack_start(&open_button, false, false, 5);

        let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        scrolled_window.set_policy(PolicyType::Automatic, PolicyType::Automatic);
        scrolled_window.set_vexpand(true);

        let drawing_area = DrawingArea::new();
        drawing_area.set_events(
            EventMask::BUTTON_PRESS_MASK | 
            EventMask::BUTTON_RELEASE_MASK | 
            EventMask::POINTER_MOTION_MASK
        );
        drawing_area.set_halign(Align::Center);
        drawing_area.set_valign(Align::Center);
        drawing_area.set_size_request(400, 300);

        // Create shared model for drawing
        let model_for_draw = Rc::new(RefCell::new(model.clone()));
        let model_for_draw_clone = model_for_draw.clone();

        // Connect draw event with shared model
        drawing_area.connect_draw(move |_, ctx| {
            let model_ref = model_for_draw_clone.borrow();
            draw_content(ctx, &*model_ref);
            Inhibit(false)
        });

        // Connect drawing events
        connect!(relm, drawing_area, connect_button_press_event(_, event), return (
            {
                let (x, y) = event.position();
                Some(Msg::StartDrawing(x, y))
            }, 
            Inhibit(false)
        ));

        connect!(relm, drawing_area, connect_motion_notify_event(_, event), return (
            {
                if event.state().contains(gtk::gdk::ModifierType::BUTTON1_MASK) {
                    let (x, y) = event.position();
                    Some(Msg::UpdateDrawing(x, y))
                } else {
                    None
                }
            }, 
            Inhibit(false)
        ));

        connect!(relm, drawing_area, connect_button_release_event(_, _), return (
            Some(Msg::FinishDrawing), 
            Inhibit(false)
        ));

        scrolled_window.add(&drawing_area);
        main_box.pack_start(&scrolled_window, true, true, 5);
        window.add(&main_box);

        connect!(relm, open_button, connect_clicked(_), Msg::OpenFile);
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));

        window.show_all();

        Win {
            model,
            widgets: Widgets {
                window,
                open_button,
                drawing_area,
                scrolled_window,
                main_box,
            },
            model_for_draw,
        }
    }
}

fn draw_content(ctx: &Context, model: &Model) {
    // Clear the area first
    ctx.set_source_rgb(1.0, 1.0, 1.0); // White background
    ctx.paint().unwrap();

    // Draw the image if available
    if let Some(ref pixbuf) = model.current_pixbuf {
        ctx.set_source_pixbuf(pixbuf, 0.0, 0.0);
        ctx.paint().unwrap();
    }

    // Draw completed rectangles
    ctx.set_source_rgb(1.0, 0.0, 0.0); // Solid red color
    ctx.set_line_width(3.0);
    
    for rect in &model.rectangles {
        ctx.rectangle(rect.x, rect.y, rect.width, rect.height);
        ctx.stroke_preserve().unwrap();
        ctx.set_source_rgb(1.0, 0.0, 0.0); // Solid red fill (fully opaque)
        ctx.fill().unwrap();
        ctx.set_source_rgb(1.0, 0.0, 0.0); // Reset to solid red for stroke
    }

    // Draw current drawing rectangle
    if let Some(ref rect) = model.drawing_rectangle {
        ctx.set_source_rgb(0.0, 0.8, 0.0); // Solid green color for current drawing
        ctx.set_line_width(2.0);
        ctx.rectangle(rect.x, rect.y, rect.width, rect.height);
        ctx.stroke().unwrap();
    }
}
