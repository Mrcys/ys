#![allow(dead_code)]
mod utils::crypto::encrypt;

const URL = "https://s9sj2.rsjp0.vercel.app/api/v1";
pub const KEY = b"OTopZbpWxlOo6WVjZH2eWh1uvJxNGp8s";
pub const IV = b"v5wiPCaxkMa8ifwK";
pub const API_URL: &str = encrypt(URL, KEY, IV);