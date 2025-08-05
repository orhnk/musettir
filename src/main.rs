use relm::{Widget, Relm, Update, connect};
use relm_derive::Msg;
use gtk::prelude::*;
use gtk::{
    Button, Label, Window, WindowType, Box as GtkBox, Orientation,
    TextView, TextBuffer, Justification, Inhibit, Align,
    FileChooserDialog, FileChooserAction, ResponseType,
};

struct Model {
    counter: u8,
}

#[derive(Msg)]
enum Msg {
    Increment,
    Decrement,
    OpenFile,
    Quit,
}

struct Win {
    model: Model,
    widgets: Widgets,
}

struct Widgets {
    window: Window,
    counter_label: Label,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            counter: 0,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Increment => {
                self.model.counter = self.model.counter.saturating_add(1);
                self.widgets.counter_label.set_text(&self.model.counter.to_string());
            }
            Msg::Decrement => {
                self.model.counter = self.model.counter.saturating_sub(1);
                self.widgets.counter_label.set_text(&self.model.counter.to_string());
            }
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

        let dec_button = Button::with_label("-");
        let counter_label = Label::new(Some(&model.counter.to_string()));
        counter_label.set_width_chars(3);
        let inc_button = Button::with_label("+");
        let open_button = Button::with_label("Open File");

        button_box.add(&dec_button);
        button_box.add(&counter_label);
        button_box.add(&inc_button);
        button_box.add(&open_button);

        let content_box = GtkBox::new(Orientation::Horizontal, 0);
        content_box.set_vexpand(true);

        let text_view = TextView::new();
        text_view.set_justification(Justification::Center);
        text_view.set_editable(false);
        
        let buffer = TextBuffer::new(None::<&gtk::TextTagTable>);
        buffer.set_text("Welcome to Musettir\n\nA simple GTK3 application.");
        text_view.set_buffer(Some(&buffer));

        content_box.add(&text_view);

        main_box.add(&button_box);
        main_box.add(&content_box);
        window.add(&main_box);

        connect!(relm, dec_button, connect_clicked(_), Msg::Decrement);
        connect!(relm, inc_button, connect_clicked(_), Msg::Increment);
        connect!(relm, open_button, connect_clicked(_), Msg::OpenFile);
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));

        window.show_all();

        Win {
            model,
            widgets: Widgets {
                window,
                counter_label,
            },
        }
    }
}

fn main() {
    relm::run::<Win>(()).unwrap();
}