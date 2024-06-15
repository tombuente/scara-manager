use eframe::egui;

pub trait Model {
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub struct ModelHandler {
    models: Vec<Box<dyn Model>>,
    open: std::collections::HashSet<String>,
}

impl Default for ModelHandler {
    fn default() -> Self {
        Self::from_models(vec![Box::<StepControl>::default()])
    }
}

impl ModelHandler {
    pub fn from_models(models: Vec<Box<dyn Model>>) -> Self {
        let open = std::collections::HashSet::new();

        Self { models, open }
    }

    pub fn checkboxes(&mut self, ui: &mut egui::Ui) {
        let Self { models, open } = self;
        for model in models {
            let mut is_open = open.contains(model.name());
            ui.toggle_value(&mut is_open, model.name());
            set_open(open, model.name(), is_open);
        }
    }

    pub fn windows(&mut self, ctx: &egui::Context) {
        let Self { models, open } = self;
        for model in models {
            let mut is_open = open.contains(model.name());
            model.show(ctx, &mut is_open);
            set_open(open, model.name(), is_open);
        }
    }
}

struct StepCommand {
    j1: i64,
    j2: i64,
}

impl Default for StepCommand {
    fn default() -> Self {
        Self { j1: 0, j2: 0 }
    }
}

pub struct StepControl {
    command: StepCommand,
}

impl Default for StepControl {
    fn default() -> Self {
        Self {
            command: StepCommand::default(),
        }
    }
}

impl Model for StepControl {
    fn name(&self) -> &'static str {
        "Step Command Builder"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Step Command Builder")
            .open(open)
            .show(ctx, |ui| {
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

fn set_open(open: &mut std::collections::HashSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}
