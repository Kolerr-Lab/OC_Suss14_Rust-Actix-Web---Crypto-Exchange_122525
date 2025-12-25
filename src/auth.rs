use std::env;
use jwt::{encode, Header, EncodingKey};

fn create_jwt(claims: &Claims) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&Header::default(), claims, &EncodingKey::from_secret(secret.as_ref())).expect("JWT encoding failed")
}