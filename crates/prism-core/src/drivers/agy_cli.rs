use async_trait::async_trait;
use lumen_model::*;
use lumen_session::AgyAdapter;
use lumen_session::SessionAdapter;
use std::io::Cursor;
use std::path::Path;
use tokio::process::Command;

use crate::{ExecutionDriver, PrismCoreError, TaskSpec};

pub struct AgyCliDriver;

#[async_trait]
impl ExecutionDriver for AgyCliDriver {
    fn name(&self) -> &'static str {
        "agy-cli"
    }

    fn is_available(&self) -> bool {
        which::which("agy").is_ok()
    }

    async fn execute(
        &self,
        task: &TaskSpec,
        sandbox_dir: &Path,
    ) -> Result<CanonicalTranscript, PrismCoreError> {
        if !self.is_available() {
            return Err(PrismCoreError::DriverUnavailable(
                "`agy` executable not found in PATH".into(),
            ));
        }

        let output = Command::new("agy")
            .arg("--non-interactive")
            .arg(&task.input_prompt)
            .current_dir(sandbox_dir)
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PrismCoreError::ProcessFailed(format!(
                "AGY CLI failed with code {:?}: {}",
                output.status.code(),
                stderr
            )));
        }

        let cursor = Cursor::new(output.stdout);
        let transcript = AgyAdapter
            .parse_stream(Box::new(cursor))
            .map_err(|e| PrismCoreError::ProcessFailed(format!("Ingestion failed: {}", e)))?;

        Ok(transcript)
    }
}
