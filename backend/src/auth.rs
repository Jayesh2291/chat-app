use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,  // subject (username or user ID)
    exp: usize,   // expiration time (in seconds)
}

const SECRET_KEY: &[u8] = b"my_secret_key";  // Secret key for JWT

// Hash a password
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

// Verify a password
pub fn verify_password(hashed_password: &str, password: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hashed_password)
}

// Generate a JWT token
pub fn generate_jwt(username: &str) -> String {
    let expiration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600;  // Token valid for 1 hour

    let claims = Claims {
        sub: username.to_string(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(SECRET_KEY);

    encode(&header, &claims, &encoding_key).unwrap()
}

// Decode a JWT token and validate it
pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(SECRET_KEY);
    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(token, &decoding_key, &validation).map(|data| data.claims)
}
