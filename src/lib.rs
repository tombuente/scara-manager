pub mod model;

use eframe::egui;

pub struct App {
    views: crate::model::ModelHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            views: crate::model::ModelHandler::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("egui_demo_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                self.menu(ui);
            });

        self.show_windows(ctx)
    }
}

impl App {
    fn menu(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                self.views.checkboxes(ui);

                ui.separator();
                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                }
            });
        });
    }

    fn show_windows(&mut self, ctx: &egui::Context) {
        self.views.windows(ctx)
    }
}
