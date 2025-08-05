use relm::{Widget, Relm, Update, connect};
use gtk::prelude::*;
use gtk::{
    Button, Window, WindowType, Box as GtkBox, Orientation,
    Inhibit, Align, FileChooserDialog, FileChooserAction, ResponseType,
};

use crate::models::Model;
use crate::messages::Msg;
use crate::widgets::Widgets;

pub struct Win {
    pub model: Model,
    pub widgets: Widgets,
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
                    Some("Open File"),
                    Some(&self.widgets.window),
                    FileChooserAction::Open,
                );
                dialog.add_button("Cancel", ResponseType::Cancel);
                dialog.add_button("Open", ResponseType::Accept);

                if dialog.run() == ResponseType::Accept {
                    if let Some(filename) = dialog.filename() {
                        println!("Selected file: {:?}", filename);
                        // Here you could read and display the file content
                    }
                }
                dialog.close();
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
        window.set_title("Musettir");
        window.set_default_size(800, 600);

        let main_box = GtkBox::new(Orientation::Vertical, 5);
        main_box.set_margin_top(5);
        main_box.set_margin_bottom(5);
        main_box.set_margin_start(5);
        main_box.set_margin_end(5);

        let button_box = GtkBox::new(Orientation::Horizontal, 5);
        button_box.set_halign(Align::Center);

        let open_button = Button::with_label("Open File");

        button_box.add(&open_button);

        main_box.add(&button_box);
        window.add(&main_box);

        connect!(relm, open_button, connect_clicked(_), Msg::OpenFile);
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));

        window.show_all();

        Win {
            model,
            widgets: Widgets {
                window,
            },
        }
    }
}
