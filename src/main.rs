use aws_credential_types::Credentials;
use aws_sdk_s3::Client as S3Client;
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
        Self {
            // cmon tell me i lEaKeD SeCrEtS kitty, pspspsps :)
            show_modal: true,
            aws_access_key_id: "GK0c36b51c9bb86e516f87f239".to_owned(),
            aws_secret_access_key:
                "029bb75840053f2dfa73cb9840bb91ba239ec27d80712e0a609116932a2b9781".to_owned(),
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

                        if ui.button("Save and Load Buckets").clicked() {
                            self.show_modal = false;

                            // Clone required data for the async task
                            let access_key = self.aws_access_key_id.clone();
                            let secret_key = self.aws_secret_access_key.clone();
                            let region = self.aws_default_region.clone();
                            let endpoint = self.aws_endpoint_url.clone();
                            // Launch the asynchronous client creation and API call
                            tokio::spawn(async move {
                                match create_s3_client(&access_key, &secret_key, &region, &endpoint)
                                    .await
                                {
                                    Ok(client) => {
                                        let arc_client = Arc::new(client);
                                        list_buckets(arc_client).await;
                                    }
                                    Err(e) => {
                                        panic!("Configuration Error: {}", e);
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

async fn create_s3_client(
    access_key: &str,
    secret_key: &str,
    region: &str,
    endpoint: &str,
) -> Result<S3Client, String> {
    if access_key.is_empty() || secret_key.is_empty() || region.is_empty() {
        return Err("Access Key, Secret Key, and Region cannot be empty.".to_owned());
    }

    let credentials = Credentials::new(access_key, secret_key, None, None, "asd");

    let mut config_loader = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .credentials_provider(credentials)
        .region(Region::new(region.to_owned()));

    if !endpoint.is_empty() {
        config_loader = config_loader.endpoint_url(endpoint);
    }

    let shared_config = config_loader.load().await;

    Ok(S3Client::new(&shared_config))
}

async fn list_buckets(client: Arc<S3Client>) {
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
}
