use eframe::egui;
use egui::ScrollArea;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Chat GPR",
        options,
        Box::new(|_cc| Box::<ChatGPR>::default()),
    )
}

struct ChatGPR {
    name: String,
    messages: Vec<String>,
    new_message: String,
}

impl Default for ChatGPR {
    fn default() -> Self {
        Self {
            name: "Kyle".to_owned(),
            messages: vec!["".to_owned()],
            new_message: "".to_owned(),
        }
    }
}

impl eframe::App for ChatGPR {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Chat GPR");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .max_height(ui.available_height() - 20.0)
                .show(ui, |ui| {
                    for message in self.messages.iter() {
                        ui.label(message);
                    }
                });
            ui.horizontal(|ui| {
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.new_message)
                        .desired_width(ui.available_width() - 50.0),
                );
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.messages
                        .push(format!("{} said: {}", self.name, self.new_message));
                    self.new_message = "".to_owned();
                    response.request_focus();
                }
                if ui.button("Send >").clicked() {
                    self.messages
                        .push(format!("{} said: {}", self.name, self.new_message));
                    self.new_message = "".to_owned();
                    response.request_focus();
                }
            });
        });
    }
}
