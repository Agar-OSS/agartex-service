use std::{process::Command, fmt::Debug, ffi::OsStr};

use axum::async_trait;
use tracing::{error, info};

#[async_trait]
pub trait ExecutionService {
    type ExecutionError;
    async fn execute<'a>(&self, comm: impl AsRef<OsStr> + Debug + Send, args: &'a [impl AsRef<OsStr> + Debug + Sync]) -> Result<String, Self::ExecutionError>;
}

#[derive(Debug, Clone)]
pub struct ProcessExecutionService;

pub enum ProcessExecutionError {
    Unknown,
    StatusError(Option<i32>, String)
}

#[async_trait]
impl ExecutionService for ProcessExecutionService {
    type ExecutionError = ProcessExecutionError;
    
    #[tracing::instrument]
    async fn execute<'a>(&self, comm: impl AsRef<OsStr> + Debug + Send, args: &'a [impl AsRef<OsStr> + Debug + Sync]) -> Result<String, Self::ExecutionError> {
        info!("Received command.");
        
        let command = Command::new(comm)
            .args(args)
            .output();


        let out = match command {
            Ok(out) => out,
            Err(err) => {
                error!(%err);
                return Err(ProcessExecutionError::Unknown);
            }
        };

        if !out.status.success() {
            match String::from_utf8(out.stdout) {
                Ok(msg) => Err(ProcessExecutionError::StatusError(out.status.code(), msg)),
                Err(err) => {
                    error!(%err);
                    Err(ProcessExecutionError::Unknown)
                }
            }
        } else if let Ok(msg) = String::from_utf8(out.stdout) {
            Ok(msg)
        } else {
            Err(ProcessExecutionError::Unknown)
        }
    }
}
