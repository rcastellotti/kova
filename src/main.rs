use aws_credential_types::Credentials;
use aws_credential_types::provider::ProvideCredentials;
use aws_types::region::Region;
use eframe::{App, Frame, NativeOptions, run_native};
use egui::{CentralPanel, Context, TextEdit};
use std::{panic, sync::Arc};

pub struct Kova {
    show_modal: bool,

    aws_access_key_id: String,
    aws_secret_access_key: String,
    aws_default_region: String,
    aws_endpoint_url: String,
}

impl Default for Kova {
    fn default() -> Self {
        // todo: can we put real values to the r2 bucket serving assets?
        Self {
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
        egui::SidePanel::left("left_panel")
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                ui.heading("buckets");
                ui.button("kova1");
                ui.button("kova2");
                ui.button("kova3");
                ui.button("kova4");
                ui.button("kova5");
            });

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

                        if ui.button("Save and Load Buckets").clicked() {
                            self.show_modal = false;

                            let access_key = self.aws_access_key_id.clone();
                            let secret_key = self.aws_secret_access_key.clone();
                            let region = self.aws_default_region.clone();
                            let endpoint = self.aws_endpoint_url.clone();

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
                                        if let Some(buckets) = output.buckets {
                                            for bucket in buckets {
                                                if let Some(name) = bucket.name {
                                                    println!("Bucket name: {}", name);
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        panic!("Error listing buckets: {:?}", e);
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
