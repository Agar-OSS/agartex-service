use std::net::{AddrParseError, SocketAddr};

#[derive(Debug)]
pub struct RawConfig {
    pub addr: &'static str,
    pub db: &'static str,
    pub hash_cost: u32
}

#[derive(Debug)]
pub struct ParsedConfig {
    pub addr: SocketAddr,
    pub db: &'static str,
    pub hash_cost: u32
}

impl RawConfig {
    #[tracing::instrument]
    pub fn verify(&self) -> Result<ParsedConfig, AddrParseError> {
        Ok(ParsedConfig {
            addr: self.addr.parse::<SocketAddr>()?,
            db: self.db,
            hash_cost: self.hash_cost
        })
    }
}

pub static SERVER_CONFIG: RawConfig = RawConfig {
    addr: "0.0.0.0:3000",
    db: "postgres://localhost:5432/agartex-db",
    hash_cost: 12
};
