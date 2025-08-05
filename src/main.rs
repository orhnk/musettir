use relm4::{
    gtk::{
        self,
        prelude::*,
        traits::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt},
        Justification, Orientation,
    },
    Component, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent,
};

struct CameraAppModel;

#[derive(Debug)]
enum CameraAppMsg {
    CaptureImage,
}

#[relm4::component]
impl SimpleComponent for CameraAppModel {
    type Init = ();
    type Input = CameraAppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Almusettir"),
            set_default_width: 800,
            set_default_height: 600,

            gtk::Box {
                gtk::Box {
                    set_orientation: Orientation::Vertical,
                    set_spacing: 5,
                    set_margin_all: 5,

                    gtk::Button {
                        set_label: "Capture",
                        connect_clicked[sender] => move |_| {
                            sender.input(CameraAppMsg::CaptureImage);
                        }
                    },

                    gtk::Button {
                        set_label: "Open File",
                        connect_clicked => move |_| {
                            println!("File dialogue button clicked. (Feature to be implemented)");
                        }
                    },
                },

                gtk::Box {
                    set_orientation: Orientation::Horizontal,

                    gtk::TextView {
                        set_justification: Justification::Center,
                        set_buffer: Some(&gtk::TextBuffer::new(None)),
                    }
                }
            },
        }
    }

    fn init(
        _: Self::Init,
        _window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = CameraAppModel;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            CameraAppMsg::CaptureImage => {
                println!("Capturing Image...");
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.camera_app");
    app.run::<CameraAppModel>(());
}
