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
        Box::new(|_cc| Box::<App>::default()),
    )
}

pub trait View {
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
    fn ui(&mut self, ui: &mut egui::Ui);
}

struct App {
    step_command_builder: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            step_command_builder: true,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        StepControl::default().show(ctx, &mut self.step_command_builder);

        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}



pub struct StepControl {
    command: StepCommand,
}

struct StepCommand {
    j1: i64,
    j2: i64,
}

impl Default for StepControl {
    fn default() -> Self {
        Self {
            command: StepCommand::default(),
        }
    }
}

impl Default for StepCommand {
    fn default() -> Self {
        Self { j1: 0, j2: 0 }
    }
}

impl View for StepControl {
    fn name(&self) -> &'static str {
        "Step Command Builder"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Step Command Builder").open(open).show(ctx, |ui| {
            self.ui(ui);
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("controls").striped(true).show(ui, |ui| {
            if ui
                .button(egui::RichText::new("+").family(egui::FontFamily::Monospace))
                .clicked()
            {
                self.command.j1 += 1;
            };
            if ui
                .button(egui::RichText::new("+").family(egui::FontFamily::Monospace))
                .clicked()
            {
                self.command.j2 += 1;
            };
            ui.end_row();

            ui.label("J1");
            ui.label("J2");
            ui.end_row();

            if ui
                .button(egui::RichText::new("-").family(egui::FontFamily::Monospace))
                .clicked()
            {
                self.command.j1 -= 1;
            };
            if ui
                .button(egui::RichText::new("-").family(egui::FontFamily::Monospace))
                .clicked()
            {
                self.command.j2 -= 1;
            };
            ui.end_row();
        });

        ui.code(format!(
            "move_steps j1={} j2={}",
            self.command.j1, self.command.j2
        ));
    }
}
