use eframe::egui;
use scara_manager::App;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320., 240.]),
        ..Default::default()
    };

    eframe::run_native(
        "Scara Manager",
        options,
        Box::new(|_cc| Box::<App>::default()),
    )
}
