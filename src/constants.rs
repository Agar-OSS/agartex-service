use std::{net::SocketAddr, str::FromStr, path::Path};

use lazy_static::lazy_static;
use regex::Regex;

pub const DB_URL: &str  = "postgres://localhost:5432/agartex-db";
pub const HASH_COST: u32 = 12;
pub const SESSION_COOKIE_NAME: &str = "RSESSID";
pub const CLIENT_URL_ENV_VAR: &str = "CLIENT_URL";
pub const SESSION_LENGTH_SECONDS: i64 = 30 * 24 * 60 * 60; // 3 months
pub const PASSWORD_SPECIAL_CHARS: &str = "!@#$%^&*";
pub const LATEXMK_PATH: &str = "latexmk";
lazy_static! {
    pub static ref COMPILE_DIR: &'static Path = Path::new("/tmp/agar_service/");
    pub static ref SERVER_URL: SocketAddr = SocketAddr::from_str("0.0.0.0:3000").unwrap();
    pub static ref PASSWORD_REGEX: Regex = Regex::new(format!("^[A-Za-z0-9{}]*$", PASSWORD_SPECIAL_CHARS).as_str()).unwrap();
}
