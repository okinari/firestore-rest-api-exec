use std::{fs::File, io::BufReader};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GoogleCredential {
    // r#type: String,
    project_id: String,
    // private_key_id: String,
    private_key: String,
    client_email: String,
    // client_id: String,
    // auth_uri: String,
    token_uri: String,
    // auth_provider_x509_cert_url: String,
    // client_x509_cert_url: String,
}

impl GoogleCredential {
	pub fn new(file_path: String) -> Self {
		let file = File::open(file_path).unwrap();
		let reader = BufReader::new(file);
		serde_json::from_reader(reader).unwrap()
	}

	pub fn get_project_id(&self) -> String {
		self.project_id.clone()
	}

	pub fn get_private_key(&self) -> String {
		self.private_key.clone()
	}

	pub fn get_client_email(&self) -> String {
		self.client_email.clone()
	}

	pub fn get_token_uri(&self) -> String {
		self.token_uri.clone()
	}
}