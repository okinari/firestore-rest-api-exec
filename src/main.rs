mod utils;
mod google;

use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use config::Config;

use google::{credential::GoogleCredential, firestore::GoogleFirestore};
use serde_json::json;
use crate::google::oauth;

#[derive(Debug, Serialize, Deserialize)]
struct Personal {
	id: Option<u32>,
	name: String,
	tel: String,
}

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
	format!("Hello {}! id:{}", name, id)
}

#[get("/list")]
async fn list() -> impl Responder {
	let firestore = setup_firestore().await;
	
	let response = firestore.get_documents("tests/bbb/sss".to_string()).await;
	println!("response: {:#?}", response);
	
	HttpResponse::Ok().body("ok")
}

#[post("/enter")]
async fn enter(personal_by_web_json: web::Json<Personal>) -> impl Responder {
	println!("post_data: {:?}", personal_by_web_json);

	let firestore = setup_firestore().await;

	let enter_data = json!({
		"fields": {
			"name": {
				"stringValue": personal_by_web_json.name,
			},
			"tel": {
				"stringValue": personal_by_web_json.tel,
			},
		},
	});

	let response = firestore.insert_document("tests/bbb/sss".to_string(), None, enter_data).await;
	println!("response: {:#?}", response);

	HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| App::new()
		.service(list))
		.bind("127.0.0.1:8080")?
		.run()
		.await
}

fn get_configs() -> Config {
	Config::builder()
		.add_source(config::File::with_name("app-config"))
		.add_source(config::Environment::with_prefix("APP"))
		.build()
		.unwrap()
}

async fn setup_firestore() -> GoogleFirestore {
	let configs = get_configs();	
	let path = configs.get_string("FIREBASE_KEY_JSON").unwrap();
	let google_credential = GoogleCredential::new(path);

	let access_token = oauth::get_access_token(&google_credential, "https://www.googleapis.com/auth/datastore".to_string()).await;

	GoogleFirestore::new(
		access_token,
		"v1".to_string(),
		google_credential.get_project_id().to_string(),
		"(default)".to_string(),
	)
}