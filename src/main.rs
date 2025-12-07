use aws_credential_types::Credentials;
use aws_credential_types::provider::ProvideCredentials;
use aws_types::region::Region;
use eframe::{App, Frame, NativeOptions, run_native};
use egui::{Context, Modal, SidePanel};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct Kova {
    buckets: Vec<String>,
    bucket_tx: Sender<Vec<String>>,
    bucket_rx: Receiver<Vec<String>>,
    show_modal: bool,
    aws_access_key_id: String,
    aws_secret_access_key: String,
    aws_default_region: String,
    aws_endpoint_url: String,
}

impl Default for Kova {
    fn default() -> Self {
        let (tx, rx) = channel();
        // todo: can we put real values to the r2 bucket serving assets?
        Self {
            buckets: Vec::new(),
            bucket_tx: tx,
            bucket_rx: rx,
            show_modal: true,
            aws_access_key_id: "GK6f5eaac85dc32ce0b9cd013c".to_owned(),
            aws_secret_access_key:
                "b97ece9a3abe8f31f5a8af960d46831ff09b1f84de1a6c9f69e58c83b34f8edf".to_owned(),
            aws_default_region: "garage".to_owned(),
            aws_endpoint_url: "http://localhost:3900".to_owned(),
        }
    }
}
impl App for Kova {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Now add your UI panels/windows on top
        egui::CentralPanel::default().show(ctx, |_ui| {
            let painter = ctx.layer_painter(egui::LayerId::background());
            let screen_rect = ctx.content_rect();
            let grid_step = 20.0;
            let color = egui::Color32::from_rgba_unmultiplied(50, 50, 50, 100);

            let mut x = screen_rect.min.x;
            while x <= screen_rect.max.x {
                painter.vline(x, screen_rect.y_range(), egui::Stroke::new(1.0, color));
                x += grid_step;
            }

            let mut y = screen_rect.min.y;
            while y <= screen_rect.max.y {
                painter.hline(screen_rect.x_range(), y, egui::Stroke::new(1.0, color));
                y += grid_step;
            }
        });

        if self.show_modal {
            Modal::new(egui::Id::new("./configure")).show(ctx, |ui| {
                ui.label("AWS_ACCESS_KEY_ID");
                ui.text_edit_singleline(&mut self.aws_access_key_id);
                ui.label("AWS_SECRET_ACCESS_KEY");
                ui.text_edit_singleline(&mut self.aws_secret_access_key);
                ui.label("AWS_DEFAULT_REGION");
                ui.text_edit_singleline(&mut self.aws_default_region);
                ui.label("AWS_ENDPOINT_URL");
                ui.text_edit_singleline(&mut self.aws_endpoint_url);

                ui.separator();
                if ui.button("./configure").clicked() {
                    self.show_modal = false;

                    let access_key = self.aws_access_key_id.clone();
                    let secret_key = self.aws_secret_access_key.clone();
                    let region = self.aws_default_region.clone();
                    let endpoint = self.aws_endpoint_url.clone();
                    let ctx_clone = ctx.clone();
                    let tx = self.bucket_tx.clone();

                    tokio::spawn(async move {
                        let credentials = Credentials::new(
                            access_key,
                            secret_key,
                            None,
                            None,
                            "StaticCredentialsProvider",
                        );
                        let credential_provider: Arc<dyn ProvideCredentials> =
                            Arc::new(credentials);

                        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
                            .region(Region::new(region))
                            .credentials_provider(credential_provider)
                            .endpoint_url(endpoint)
                            .load()
                            .await;

                        let client = aws_sdk_s3::Client::new(&config);
                        match client.list_buckets().send().await {
                            Ok(output) => {
                                let names: Vec<String> = output
                                    .buckets
                                    .unwrap_or_default()
                                    .into_iter()
                                    .filter_map(|b| b.name)
                                    .collect();
                                let _ = tx.send(names);
                                ctx_clone.request_repaint();
                            }
                            Err(e) => {
                                eprintln!("Error listing buckets: {:?}", e);
                            }
                        }
                    });
                }
            });
            return;
        }
        if let Ok(new_buckets) = self.bucket_rx.try_recv() {
            self.buckets = new_buckets;
        }

        SidePanel::left("left_panel")
            .default_width(150.0)
            .resizable(false)
            .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                ui.heading("buckets");
                if !self.buckets.is_empty() {
                    for bucket_name in &self.buckets {
                        if ui.button(bucket_name).clicked() {
                            println!("Selected bucket: {}", bucket_name);
                        }
                    }
                }
            });
    }
}

#[::tokio::main]
async fn main() -> eframe::Result<()> {
    let native_options = NativeOptions::default();

    run_native(
        "kova",
        native_options,
        Box::new(|_cc| Ok(Box::new(Kova::default()))),
    )
}
