use std::collections::HashMap;
use actix_web::client::Client;
use serde_json::Value;

pub async fn get_json(url: String, headers: Option<HashMap<String, String>>) -> String {
	let mut request = Client::default()
		.get(url);
		
	if let Some(h) = headers {
		for (key, val) in h.iter() {
			request = request.header(key, val.to_string());
		}
	}

	let response = request
		.send()
		.await
		.unwrap()
		.body()
		.limit(6400)
		.await
		.unwrap();

	let str_resp = response.iter().map(|&s| s as char).collect::<String>();
	return str_resp;
}

pub async fn post_json(url: String, headers: Option<HashMap<String, String>>, json: Value) -> String {
	let mut request = Client::default()
		.post(url);

	if let Some(h) = headers {
		for (key, val) in h.iter() {
			request = request.header(key.to_string(), val.to_string());
		}
	}

	let response = request
		.send_json(&json)
		.await
		.unwrap()
		.body()
		.limit(6400)
		.await
		.unwrap();

	let str_resp = response.iter().map(|&s| s as char).collect::<String>();
	return str_resp;
}
