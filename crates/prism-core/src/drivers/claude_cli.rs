use async_trait::async_trait;
use lumen_model::*;
use lumen_session::ClaudeCodeAdapter;
use lumen_session::SessionAdapter;
use std::io::Cursor;
use std::path::Path;
use tokio::process::Command;

use crate::{ExecutionDriver, PrismCoreError, TaskSpec};

pub struct ClaudeCliDriver;

#[async_trait]
impl ExecutionDriver for ClaudeCliDriver {
    fn name(&self) -> &'static str {
        "claude-cli"
    }

    fn is_available(&self) -> bool {
        which::which("claude").is_ok()
    }

    async fn execute(
        &self,
        task: &TaskSpec,
        sandbox_dir: &Path,
    ) -> Result<CanonicalTranscript, PrismCoreError> {
        if !self.is_available() {
            return Err(PrismCoreError::DriverUnavailable(
                "`claude` executable not found in PATH".into(),
            ));
        }

        let output = Command::new("claude")
            .arg("-p")
            .arg(&task.input_prompt)
            .arg("--output-format=json")
            .current_dir(sandbox_dir)
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PrismCoreError::ProcessFailed(format!(
                "Claude CLI failed with code {:?}: {}",
                output.status.code(),
                stderr
            )));
        }

        let cursor = Cursor::new(output.stdout);
        let transcript = ClaudeCodeAdapter
            .parse_stream(Box::new(cursor))
            .map_err(|e| PrismCoreError::ProcessFailed(format!("Ingestion failed: {}", e)))?;

        Ok(transcript)
    }
}
