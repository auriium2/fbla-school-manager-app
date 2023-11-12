pub mod app;
pub mod ui;

use eframe::{App, AppCreator, CreationContext, egui};
use crate::app::Application;
use crate::ui::main_window::MainWindow;


fn main() {
    let opts = eframe::NativeOptions::default();

    //TODO more option stuff, etc etc
    eframe::run_native("", opts, Box::new(start_app))
}

fn start_app(ctx: &CreationContext) -> Box<dyn App> {

    return Box::new(Application { window: MainWindow::new(0.56) })
}
