use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions{
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "ruix",
        options,
        Box::new(|_cc| Box::<App>::default()),
        )
}

struct App {
    name: String,
    age: u32,
    show_confirmation_dialog: bool,
    close: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            name: "Daniel".to_owned(),
            age: 26,
            show_confirmation_dialog: false,
            close: false,
        }
    }
}

impl eframe::App for App {
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RUIX");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

        });

        if self.show_confirmation_dialog {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, | ui | {
                    ui.horizontal(| ui | {
                        if ui.button("Cancel").clicked() {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.close = true;
                            frame.close();
                        }
                    });
                });
        }
    }
}
