mod proto {
    tonic::include_proto!("scaraproto");
}

use eframe::egui;
use proto::scara_client::ScaraClient;
use std::sync::{Arc, Mutex};

trait Model {
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::Context, open: &mut bool, state: &mut State);
    fn ui(&mut self, ui: &mut egui::Ui, state: &mut State);
}

pub struct App {
    state: State,

    model_handler: ModelHandler,

    // gRPC
    client: ScaraClient<tonic::transport::Channel>,
    runtime: tokio::runtime::Runtime,
    response: Arc<Mutex<String>>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("nav")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                self.menu(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing();
            ui.label("Response form server:");
            ui.label(format!("{}", *self.response.lock().unwrap()));

            if ui.button("Ping").clicked() {
                self.spawn_upload_program_grpc();
            }

            for cmd in &self.state.program {
                ui.code(cmd);
            }
        });

        self.show_windows(ctx)
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().expect("Couldn't create multi-thread runtime");
        let client = runtime.block_on(create_connection());

        Self {
            state: State::default(),
            model_handler: ModelHandler::default(),

            // gRPC
            client: client,
            runtime: runtime,
            response: Arc::new(Mutex::new(String::from("-"))),
        }
    }

    fn menu(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                self.model_handler.checkboxes(ui);

                ui.separator();
                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                }
            });
        });
    }

    fn show_windows(&mut self, ctx: &egui::Context) {
        self.model_handler.windows(ctx, &mut self.state)
    }

    fn spawn_upload_program_grpc(&mut self) {
        let response_ref = self.response.clone();
        let client = self.client.clone();

        self.runtime.spawn(async move {
            Self::upload_program_grpc(response_ref, client).await
        });
    }

    async fn upload_program_grpc(response: Arc<Mutex<String>>, mut client: ScaraClient<tonic::transport::Channel>) {
        let request = proto::UploadProgramRequest {program: "program string".to_owned()};

        match client.upload_program(request).await {
            Ok(_) => *response.lock().unwrap() = "Successful gRPC request".to_owned(),
            Err(e) => println!("grpc request error: {}", e.to_string())
        }
    }
}

struct State {
    program: Vec<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            program: Vec::new(),
        }
    }
}

struct ModelHandler {
    models: Vec<Box<dyn Model>>,
    open: std::collections::HashSet<String>,
}

impl Default for ModelHandler {
    fn default() -> Self {
        Self::from_models(vec![Box::<StepControlModel>::default()])
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

    pub fn windows(&mut self, ctx: &egui::Context, state: &mut State) {
        let Self { models, open } = self;
        for model in models {
            let mut is_open = open.contains(model.name());
            model.show(ctx, &mut is_open, state);
            set_open(open, model.name(), is_open);
        }
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

struct StepControlModel {
    j1: i64,
    j2: i64,
}

impl Default for StepControlModel {
    fn default() -> Self {
        Self { j1: 0, j2: 0 }
    }
}

impl Model for StepControlModel {
    fn name(&self) -> &'static str {
        "Step Command Builder"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool, state: &mut State) {
        egui::Window::new("Step Command Builder")
            .open(open)
            .show(ctx, |ui| {
                self.ui(ui, state);
            });
    }

    fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        egui::Grid::new("controls").striped(true).show(ui, |ui| {
            if ui
                .button(egui::RichText::new("+").family(egui::FontFamily::Monospace))
                .clicked()
            {
                self.j1 += 1;
            };
            if ui
                .button(egui::RichText::new("+").family(egui::FontFamily::Monospace))
                .clicked()
            {
                self.j2 += 1;
            };
            ui.end_row();

            ui.label("J1");
            ui.label("J2");
            ui.end_row();

            if ui
                .button(egui::RichText::new("-").family(egui::FontFamily::Monospace))
                .clicked()
            {
                self.j1 -= 1;
            };
            if ui
                .button(egui::RichText::new("-").family(egui::FontFamily::Monospace))
                .clicked()
            {
                self.j2 -= 1;
            };
            ui.end_row();
        });

        ui.code(format!("j1={} j2={}", self.j1, self.j2));

        if ui.button("Send").clicked() {
            state.program.push(format!("j1={} j2={}", self.j1, self.j2));
        };
    }
}

async fn create_connection() -> ScaraClient<tonic::transport::Channel> {
    ScaraClient::connect("http://localhost:5000").await.expect("Couldn't connect to gRPC server")
}
