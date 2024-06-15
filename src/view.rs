pub trait View {
    fn name(&self) -> &'static str;

    fn update(&mut self, ui: &mut egui::Ui);
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
