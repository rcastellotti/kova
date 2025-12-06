use eframe::{App, Frame, NativeOptions, run_native};
use egui::{CentralPanel, Context, TextEdit};

pub struct Kova {
    show_modal: bool,

    aws_access_key_id: String,
    aws_secret_access_key: String,
    aws_default_region: String,
    aws_endpoint_url: String,
}

impl Default for Kova {
    fn default() -> Self {
        Self {
            show_modal: true,
            aws_access_key_id: "".to_owned(),
            aws_secret_access_key: "".to_owned(),
            aws_default_region: "garage".to_owned(),
            aws_endpoint_url: "http://localhost:3900".to_owned(),
        }
    }
}

impl App for Kova {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                if ui.button("./configure").clicked() {
                    self.show_modal = true
                }
                if self.show_modal {
                    egui::Modal::new(egui::Id::new("./configure")).show(ctx, |ui| {
                        ui.label("AWS_ACCESS_KEY_ID");
                        ui.add(TextEdit::singleline(&mut self.aws_access_key_id));
                        ui.label("AWS_SECRET_ACCESS_KEY");
                        ui.add(
                            TextEdit::singleline(&mut self.aws_secret_access_key).password(true),
                        );
                        ui.label("AWS_DEFAULT_REGION");
                        ui.add(TextEdit::singleline(&mut self.aws_default_region));
                        ui.label("AWS_ENDPOINT_URL");
                        ui.add(TextEdit::singleline(&mut self.aws_endpoint_url));

                        ui.separator();

                        if ui.button("Save").clicked() {
                            self.show_modal = false;
                        }
                    });
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = NativeOptions::default();

    run_native(
        "kova",
        native_options,
        Box::new(|_cc| Ok(Box::new(Kova::default()))),
    )
}
