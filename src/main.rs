use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320., 240.]),
        ..Default::default()
    };

    eframe::run_native(
        "Scara Manager",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    step_command: StepCommand,
}

struct StepCommand {
    j1: i64,
    j2: i64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            step_command: StepCommand::default(),
        }
    }
}

impl Default for StepCommand {
    fn default() -> Self {
        Self { j1: 0, j2: 0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Step Command Builder")
            .movable(false)
            .show(ctx, |ui| {
                egui::Grid::new("controls").striped(true).show(ui, |ui| {
                    if ui
                        .button(egui::RichText::new("+").family(egui::FontFamily::Monospace))
                        .clicked()
                    {
                        self.step_command.j1 += 1;
                    };
                    if ui
                        .button(egui::RichText::new("+").family(egui::FontFamily::Monospace))
                        .clicked()
                    {
                        self.step_command.j2 += 1;
                    };
                    ui.end_row();

                    ui.label("J1");
                    ui.label("J2");
                    ui.end_row();

                    if ui
                        .button(egui::RichText::new("-").family(egui::FontFamily::Monospace))
                        .clicked()
                    {
                        self.step_command.j1 -= 1;
                    };
                    if ui
                        .button(egui::RichText::new("-").family(egui::FontFamily::Monospace))
                        .clicked()
                    {
                        self.step_command.j2 -= 1;
                    };
                    ui.end_row();
                });

                ui.code(format!(
                    "move_steps j1={} j2={}",
                    self.step_command.j1, self.step_command.j2
                ));
            });

        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}
