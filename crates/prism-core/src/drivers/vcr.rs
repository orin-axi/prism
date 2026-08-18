use async_trait::async_trait;
use lumen_model::CanonicalTranscript;
use std::path::{Path, PathBuf};

use crate::cassette::VcrCassette;
use crate::{ExecutionDriver, PrismCoreError, TaskSpec};

pub struct VcrReplayDriver {
    cassettes_dir: PathBuf,
}

impl VcrReplayDriver {
    pub fn new(cassettes_dir: PathBuf) -> Self {
        Self { cassettes_dir }
    }
}

#[async_trait]
impl ExecutionDriver for VcrReplayDriver {
    fn name(&self) -> &'static str {
        "vcr-replay"
    }

    fn is_available(&self) -> bool {
        self.cassettes_dir.exists()
    }

    async fn execute(
        &self,
        task: &TaskSpec,
        _sandbox_dir: &Path,
    ) -> Result<CanonicalTranscript, PrismCoreError> {
        let cassette_path = self.cassettes_dir.join(format!("{}.json", task.id));
        if !cassette_path.exists() {
            return Err(PrismCoreError::CassetteNotFound(format!(
                "Cassette not found at {}",
                cassette_path.display()
            )));
        }

        let cassette = VcrCassette::load_from_file(&cassette_path)?;
        Ok(cassette.transcript)
    }
}
