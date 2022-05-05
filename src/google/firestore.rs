use std::collections::HashMap;

use serde_json::Value;

use crate::utils::http;

fn get_firestore_url(version: String, project_id: String, database_id: String, pattern: String) -> String {
	format!("https://firestore.googleapis.com/{}/projects/{}/databases/{}/documents/{}", version, project_id, database_id, pattern)
}

pub struct GoogleFirestore {
	access_token: String,
	version: String,
	project_id : String,
	database_id: String,
}

impl GoogleFirestore {
	pub fn new(access_token: String, version: String, project_id: String, database_id: String) -> Self {
		GoogleFirestore {
			access_token: access_token,
			version: version,
			project_id: project_id,
			database_id: database_id,
		}
	}

	fn get_access_token(&self) -> String {
		self.access_token.clone()
	}

	fn get_version(&self) -> String {
		self.version.clone()
	}

	fn get_project_id(&self) -> String {
		self.project_id.clone()
	}

	fn get_database_id(&self) -> String {
		self.database_id.clone()
	}

	pub async fn get_documents(&self, pattern: String) -> String {
		let url = get_firestore_url(
			self.get_version(),
			self.get_project_id(),
			self.get_database_id(),
			pattern,
		);
		
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Accept".to_string(), "application/json".to_string());
		headers.insert("Authorization".to_string(), self.get_access_token());

		http::get_json(url, Some(headers)).await
	}

	pub async fn insert_document(&self, pattern: String, document_id: Option<String>, params: Value) -> String {
		let mut url = get_firestore_url(
			self.get_version(),
			self.get_project_id(),
			self.get_database_id(),
			pattern,
		);
		match document_id {
			None => {},
			Some(str) => {
				url.push_str("?documentId=");
				url.push_str(str.as_str());
			},
		}
		
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Accept".to_string(), "application/json".to_string());
		headers.insert("Authorization".to_string(), self.get_access_token());

		let body_json: Value = serde_json::from_str(&generate_object(&params)).ok().unwrap();
		http::post_json(url, Some(headers), body_json).await
	}
}

fn generate_values(params: &Value) -> String {
	if params.is_array() {
		let tmp = params.as_array().unwrap();
		let mut vec = vec![];
		for val in tmp {
			vec.push(format!("{}", generate_values(val)));
		}
		return format!("{{\"arrayValue\": {{\"values\": [ {} ] }} }}", vec.join(","));
	}
	else if params.is_boolean() {
		return format!("{{\"booleanValue\": {} }}", params.as_bool().unwrap());
	}
	else if params.is_f64() {
		return format!("{{\"doubleValue\": {} }}", params.as_f64().unwrap());
	}
	else if params.is_i64() {
		return format!("{{\"integerValue\": {} }}", params.as_i64().unwrap());
	}
	else if params.is_null() {
		return String::from("{{\"nullValue\": \"NULL_VALUE\"}}");
	}
	else if params.is_object() {
		return format!("{{\"mapValue\": {} }}", generate_object(params));
	}
	else if params.is_string() {
		return format!("{{\"stringValue\": \"{}\" }}", params.as_str().unwrap());
	}

	// error
	"".to_string()
}

fn generate_object(params: &Value) -> String {
	let tmp = params.as_object().unwrap();
	let mut vec = vec![];
	for( key, val) in tmp.iter() {
		vec.push(format!("\"{}\": {}", key, generate_values(val)));
	}
	format!("{{\"fields\": {{ {} }} }}", vec.join(","))
}