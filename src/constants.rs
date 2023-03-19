pub const SERVER_URL: ([u8; 4], u16) = ([0, 0, 0, 0], 3000);
pub const DB_URL: &str  = "postgres://localhost:5432/agartex-db";
pub const HASH_COST: u32 = 12;
pub const SESSIONS_TABLE: &str = "sessions";
pub const SESSION_ID: &str = "RSESSID";
