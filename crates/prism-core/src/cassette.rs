use compact_str::CompactString;
use lumen_model::CanonicalTranscript;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use crate::PrismCoreError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VcrCassette {
    pub task_id: CompactString,
    pub prompt_hash: CompactString,
    pub transcript: CanonicalTranscript,
}

impl VcrCassette {
    pub fn compute_prompt_hash(system_prompt: &str, messages_prefix: &str, tools_str: &str) -> CompactString {
        let mut hasher = Sha256::new();
        hasher.update(system_prompt.as_bytes());
        hasher.update(b"||");
        hasher.update(messages_prefix.as_bytes());
        hasher.update(b"||");
        hasher.update(tools_str.as_bytes());
        let hash = hasher.finalize();
        CompactString::new(hex::encode(hash))
    }

    pub fn load_from_file(path: &Path) -> Result<Self, PrismCoreError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let cassette: Self = serde_json::from_reader(reader)
            .map_err(|e| PrismCoreError::ProcessFailed(format!("Failed to parse cassette: {}", e)))?;
        Ok(cassette)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), PrismCoreError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)
            .map_err(|e| PrismCoreError::ProcessFailed(format!("Failed to save cassette: {}", e)))?;
        Ok(())
    }
}
