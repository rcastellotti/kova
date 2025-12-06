use aws_credential_types::Credentials;
use aws_credential_types::provider::ProvideCredentials;
use aws_types::region::Region;
use eframe::{App, Frame, NativeOptions, run_native};
use egui::{CentralPanel, Context, Modal, SidePanel, TextEdit};
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
        if let Ok(new_buckets) = self.bucket_rx.try_recv() {
            self.buckets = new_buckets;
        }

        SidePanel::left("left_panel")
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                ui.heading("buckets");
                if self.buckets.is_empty() {
                    ui.label("no buckets loaded");
                } else {
                    for bucket_name in &self.buckets {
                        if ui.button(bucket_name).clicked() {
                            println!("Selected bucket: {}", bucket_name);
                        }
                    }
                }
            });

        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                if ui.button("./configure").clicked() {
                    self.show_modal = true
                }

                if self.show_modal {
                    Modal::new(egui::Id::new("./configure")).show(ctx, |ui| {
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

                        if ui.button("Save and Load Buckets").clicked() {
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

                                let config =
                                    aws_config::defaults(aws_config::BehaviorVersion::latest())
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
                }
            });
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
