//! Trace storage.

use super::Trace;
use crate::error::TraceError;
use crate::{Error, Result};
use directories::ProjectDirs;
use std::path::PathBuf;

/// Storage for traces.
pub struct TraceStore {
    traces_dir: PathBuf,
}

impl TraceStore {
    /// Create a new trace store.
    pub fn new() -> Result<Self> {
        let traces_dir = ProjectDirs::from("", "", "vibelings")
            .map(|dirs| dirs.data_dir().join("traces"))
            .unwrap_or_else(|| PathBuf::from(".vibelings/traces"));

        // Ensure directory exists
        std::fs::create_dir_all(&traces_dir)?;

        Ok(Self { traces_dir })
    }

    /// Save a trace and return its ID.
    pub fn save(&self, trace: &Trace) -> Result<String> {
        let path = self.traces_dir.join(format!("{}.json", trace.id));
        let content = serde_json::to_string_pretty(trace)?;
        std::fs::write(&path, content)?;
        Ok(trace.id.clone())
    }

    /// Save a trace from raw data (simplified interface).
    pub fn save_trace(
        &self,
        exercise_id: &str,
        prompt: &str,
        response: &str,
        passed: bool,
        duration_secs: f64,
    ) -> Result<String> {
        let mut trace = Trace::new(exercise_id);
        trace.add_message("system", prompt);
        trace.set_response(response);
        trace.complete(passed, duration_secs);
        self.save(&trace)
    }

    /// Load a trace by ID.
    pub fn load(&self, trace_id: &str) -> Result<Trace> {
        let path = self.traces_dir.join(format!("{}.json", trace_id));

        if !path.exists() {
            return Err(Error::Trace(TraceError::NotFound(trace_id.to_string())));
        }

        let content = std::fs::read_to_string(&path)?;
        let trace: Trace = serde_json::from_str(&content)
            .map_err(|e| Error::Trace(TraceError::InvalidFormat(e.to_string())))?;

        Ok(trace)
    }

    /// List all traces for an exercise.
    pub fn list_for_exercise(&self, exercise_id: &str) -> Result<Vec<String>> {
        let mut trace_ids = Vec::new();

        for entry in std::fs::read_dir(&self.traces_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(trace) = serde_json::from_str::<Trace>(&content) {
                        if trace.exercise_id == exercise_id {
                            trace_ids.push(trace.id);
                        }
                    }
                }
            }
        }

        Ok(trace_ids)
    }

    /// Get the most recent trace for an exercise.
    pub fn get_latest(&self, exercise_id: &str) -> Result<Option<Trace>> {
        let mut latest: Option<Trace> = None;

        for entry in std::fs::read_dir(&self.traces_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(trace) = serde_json::from_str::<Trace>(&content) {
                        if trace.exercise_id == exercise_id {
                            match &latest {
                                None => latest = Some(trace),
                                Some(current) => {
                                    if trace.timestamp > current.timestamp {
                                        latest = Some(trace);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(latest)
    }

    /// Delete old traces, keeping only the N most recent per exercise.
    pub fn cleanup(&self, keep_per_exercise: usize) -> Result<usize> {
        use std::collections::HashMap;

        // Group traces by exercise
        let mut traces_by_exercise: HashMap<String, Vec<Trace>> = HashMap::new();

        for entry in std::fs::read_dir(&self.traces_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(trace) = serde_json::from_str::<Trace>(&content) {
                        traces_by_exercise
                            .entry(trace.exercise_id.clone())
                            .or_default()
                            .push(trace);
                    }
                }
            }
        }

        let mut deleted = 0;

        // For each exercise, sort by timestamp and delete old ones
        for (_, mut traces) in traces_by_exercise {
            traces.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

            for trace in traces.into_iter().skip(keep_per_exercise) {
                let path = self.traces_dir.join(format!("{}.json", trace.id));
                if std::fs::remove_file(&path).is_ok() {
                    deleted += 1;
                }
            }
        }

        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_trace_store() {
        let temp_dir = TempDir::new().unwrap();
        let store = TraceStore {
            traces_dir: temp_dir.path().to_path_buf(),
        };

        let trace = Trace::new("test/exercise");
        let id = store.save(&trace).unwrap();

        let loaded = store.load(&id).unwrap();
        assert_eq!(loaded.exercise_id, "test/exercise");
    }
}
