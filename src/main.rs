extern crate sha2;
extern crate data_encoding;

use uuid::Uuid;
use sha2::{Sha256, Digest};
use ring::hmac;
use data_encoding::BASE64;

fn main(){
    println!("{}", std::env!("SECRET"));
    let message: String = "djdjdjdjdj".to_string();
    let my_uuid = Uuid::new_v4();
    println!("{}", my_uuid.to_hyphenated().to_string());
    let mut hasher = Sha256::new();
    let seed = format!("{}|{}", my_uuid.to_hyphenated().to_string(), message);
    println!("{}", seed);
    hasher.update(seed);
    let result = hasher.finalize();
    println!("sha256: {}", format!("{:X}", result));

    let salt = "b6bc6637-98ea-4851-88df-556a3e0871b8";
    let key = hmac::Key::new(hmac::HMAC_SHA512, salt.as_bytes());
    let mac = hmac::sign(&key, format!("{:X}", result).as_bytes());
    let b64_encoded_sig = BASE64.encode(mac.as_ref());
    println!("hmacSha256: {}", b64_encoded_sig);
}
