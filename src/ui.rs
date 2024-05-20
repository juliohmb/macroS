use eframe::egui;
use super::recorder;
use super::script_generator;

pub struct MacroRecorder {
    recorder: recorder::Recorder,
    status: String,
    macros: Vec<(String, String)>, // Nome da macro e teclas de atalho
}

impl MacroRecorder {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for MacroRecorder {
    fn default() -> Self {
        Self {
            recorder: recorder::Recorder::new(),
            status: String::from("Idle"),
            macros: vec![("Macro1".to_string(), "CTRL+ALT+M".to_string())],
        }
    }
}

impl eframe::App for MacroRecorder {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut status_update = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            // Título da aplicação
            ui.heading("MACRO-S");

            // Mensagens de status
            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.label(&self.status);
            });

            ui.separator();

            // Botões de controle
            ui.horizontal(|ui| {
                if ui.button("START").clicked() {
                    self.recorder.start();
                    self.status = String::from("Recording...");
                }

                if ui.button("STOP").clicked() {
                    self.recorder.stop();
                    self.status = String::from("Stopped");
                    let actions = self.recorder.get_actions();
                    if let Err(e) = script_generator::generate_python_script(actions) {
                        self.status = format!("Error: {}", e);
                    } else {
                        self.status = String::from("Script generated!");
                    }
                }
            });

            ui.separator();

            // Lista de macros
            ui.heading("Macros");
            for (_index, (name, shortcut)) in self.macros.iter().enumerate() {
                let name_clone = name.clone();
                let shortcut_clone = shortcut.clone();
                ui.horizontal(|ui| {
                    ui.label(&name_clone);
                    ui.label(&shortcut_clone);
                    if ui.button("▶").clicked() {
                        status_update = Some(format!("Executing {}...", name_clone));
                        // Aqui você pode adicionar a lógica para executar a macro
                    }
                });
            }
        });

        if let Some(new_status) = status_update {
            self.status = new_status;
        }
    }
}
