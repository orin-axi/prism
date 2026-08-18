use async_trait::async_trait;
use compact_str::CompactString;
use lumen_model::CanonicalTranscript;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

pub mod cassette;
pub mod drivers;

pub use cassette::VcrCassette;
pub use drivers::{AgyCliDriver, ClaudeCliDriver, OpenAiCompatibleDriver, OpenCodeDriver, VcrReplayDriver};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_constructors() {
        let qwen = OpenAiCompatibleDriver::qwen_coder("qwen2.5-coder-32b-instruct");
        assert_eq!(qwen.name(), "openai-compatible");
        assert_eq!(qwen.model_name, "qwen2.5-coder-32b-instruct");

        let kimi = OpenAiCompatibleDriver::kimi("moonshot-v1-128k");
        assert_eq!(kimi.model_name, "moonshot-v1-128k");

        let glm = OpenAiCompatibleDriver::glm("glm-4-plus");
        assert_eq!(glm.model_name, "glm-4-plus");

        let gemini = OpenAiCompatibleDriver::gemini("gemini-2.0-flash");
        assert_eq!(gemini.model_name, "gemini-2.0-flash");

        let opencode = OpenCodeDriver::default();
        assert_eq!(opencode.name(), "opencode");
    }
}

#[derive(Error, Debug)]
pub enum PrismCoreError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Driver unavailable: {0}")]
    DriverUnavailable(String),
    #[error("Cassette not found for task {0}")]
    CassetteNotFound(String),
    #[error("Execution process failed: {0}")]
    ProcessFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSpec {
    pub id: CompactString,
    pub skill: CompactString,
    pub input_prompt: String,
    pub fixture_repo: Option<PathBuf>,
    pub expected_assertions: Vec<CompactString>,
    pub max_turns: usize,
    pub timeout_seconds: u64,
}

#[async_trait]
pub trait ExecutionDriver: Send + Sync {
    fn name(&self) -> &'static str;
    fn is_available(&self) -> bool;
    async fn execute(
        &self,
        task: &TaskSpec,
        sandbox_dir: &Path,
    ) -> Result<CanonicalTranscript, PrismCoreError>;
}

pub struct WorkspaceSandbox {
    _temp_dir: tempfile::TempDir,
    pub path: PathBuf,
}

impl WorkspaceSandbox {
    pub fn new(fixture_path: &Path) -> Result<Self, std::io::Error> {
        let temp_dir = tempfile::tempdir()?;
        let path = temp_dir.path().to_path_buf();

        if fixture_path.exists() {
            let _ = std::process::Command::new("cp")
                .arg("-r")
                .arg(fixture_path)
                .arg(&path)
                .output()?;
        }

        // Initialize git
        let _ = std::process::Command::new("git")
            .arg("init")
            .current_dir(&path)
            .output()?;

        Ok(Self {
            _temp_dir: temp_dir,
            path,
        })
    }
}
