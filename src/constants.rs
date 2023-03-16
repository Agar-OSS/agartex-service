use std::net::{AddrParseError, SocketAddr};

pub struct RawConfig {
    pub addr: &'static str,
}

pub struct ParsedConfig {
    pub addr: SocketAddr,
}

impl RawConfig {
    pub fn verify(&self) -> Result<ParsedConfig, AddrParseError> {
        Ok(ParsedConfig {
            addr: self.addr.parse::<SocketAddr>()?,
        })
    }
}

pub static SERVER_CONFIG: RawConfig = RawConfig {
    addr: "0.0.0.0:3000",
};
