use std::path::PathBuf;

#[derive(Clone, relm_derive::Msg)]
pub enum Msg {
    OpenFile,
    LoadImage(PathBuf),
    StartDrawing(f64, f64),
    UpdateDrawing(f64, f64),
    FinishDrawing,
    ColorChanged(f64, f64, f64), // RGB values
    Quit,
}
