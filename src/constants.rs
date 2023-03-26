pub const SERVER_URL: ([u8; 4], u16) = ([0, 0, 0, 0], 3000);
pub const DB_URL: &str  = "postgres://localhost:5432/agartex-db";
pub const HASH_COST: u32 = 12;
pub const SESSION_COOKIE_NAME: &str = "RSESSID";
pub const CLIENT_URL: &str = "http://localhost:5000";
pub const SESSION_LENGTH_SECONDS: i64 = 30 * 24 * 60 * 60; // 3 months
