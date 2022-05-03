use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::utils::http;
use crate::google::credential::GoogleCredential;

#[derive(Debug, Serialize)]
struct Claims {
	iss: String,
	scope: String,
	aud: String,
	exp: i64,
	iat: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Oauth {
	access_token: String,
	expires_in: u32,
	token_type: String,
}

pub async fn get_access_token(google_credential: &GoogleCredential, scope: String) -> String {
	let mut header = Header::default();
	header.typ = Some("JWT".to_string());
	header.alg = Algorithm::RS256;
	
	let now = Utc::now();
	let iat = now.timestamp();
	let exp = (now + Duration::minutes(60)).timestamp();

	let my_claims = Claims {
		iss: google_credential.get_client_email().to_string(),
		scope: scope,
		aud: google_credential.get_token_uri().to_string(),
		exp: exp,
		iat: iat,
	};
	
	let private_key = google_credential.get_private_key().to_string();
	let jwt = encode(&header, &my_claims, &EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap()).unwrap();
	
	let token_body = json!({
		"grant_type": "urn:ietf:params:oauth:grant-type:jwt-bearer",
		"assertion": jwt
	});
	
	let result = http::post_json(my_claims.aud, None, token_body).await;
	let oauth_json: Result<Oauth, serde_json::Error> = serde_json::from_str(&result);
	let oauth = oauth_json.ok().unwrap();

	format!("{} {}", oauth.token_type, oauth.access_token)
}