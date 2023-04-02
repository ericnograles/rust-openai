use actix_web::HttpRequest;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
}

pub fn validate_jwt(request: &HttpRequest) -> Result<Claims, ()> {
    let auth_header = request.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        if let Ok(token) = header_value.to_str() {
            let token = token.trim_start_matches("Bearer ");
            let secret = std::env::var("AUTH0_SECRET").expect("AUTH0_SECRET must be set");
            let key = DecodingKey::from_secret(secret.as_ref());
            let validation = Validation::new(Algorithm::HS256);
            let decoded = decode::<Claims>(token, &key, &validation);

            return decoded.map(|data| data.claims).map_err(|_| ());
        }
    }

    Err(())
}
