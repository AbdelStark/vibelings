//! Exercise discovery and management.

use crate::config::{load_or_create_config, load_progress, save_progress, ExerciseProgress};
use crate::error::ExerciseError;
use crate::exercise::{Exercise, ExerciseManifest, ExerciseStatus};
use crate::grader::Grader;
use crate::provider::create_provider;
use crate::trace::TraceStore;
use crate::{Error, Result};
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

use super::RunResult;

/// The exercise runner, responsible for discovering and running exercises.
pub struct ExerciseRunner {
    exercises_dir: PathBuf,
    grader: Grader,
    trace_store: TraceStore,
}

impl ExerciseRunner {
    /// Create a new exercise runner.
    pub fn new() -> Result<Self> {
        let exercises_dir = PathBuf::from("exercises");
        let grader = Grader::new()?;
        let trace_store = TraceStore::new()?;

        Ok(Self {
            exercises_dir,
            grader,
            trace_store,
        })
    }

    /// Discover all exercises in the exercises directory.
    pub fn discover_exercises(&self) -> Result<Vec<Exercise>> {
        let mut exercises = Vec::new();

        if !self.exercises_dir.exists() {
            return Ok(exercises);
        }

        for entry in WalkDir::new(&self.exercises_dir)
            .min_depth(2)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join("manifest.toml");
                if manifest_path.exists() {
                    match self.load_exercise(path) {
                        Ok(exercise) => exercises.push(exercise),
                        Err(e) => {
                            eprintln!("Warning: Failed to load exercise at {:?}: {}", path, e);
                        }
                    }
                }
            }
        }

        // Sort by track and then by id
        exercises.sort_by(|a, b| {
            let track_cmp = a
                .manifest
                .exercise
                .track
                .dir_name()
                .cmp(b.manifest.exercise.track.dir_name());
            if track_cmp == std::cmp::Ordering::Equal {
                a.manifest.exercise.id.cmp(&b.manifest.exercise.id)
            } else {
                track_cmp
            }
        });

        Ok(exercises)
    }

    /// Load an exercise from a directory.
    fn load_exercise(&self, path: &Path) -> Result<Exercise> {
        let manifest_path = path.join("manifest.toml");
        let manifest_content = std::fs::read_to_string(&manifest_path)?;
        let manifest: ExerciseManifest = toml::from_str(&manifest_content).map_err(|e| {
            Error::Exercise(ExerciseError::InvalidManifest {
                path: manifest_path.clone(),
                reason: e.to_string(),
            })
        })?;

        let progress = load_progress().unwrap_or_default();
        let full_id = format!(
            "{}/{}",
            manifest.exercise.track.dir_name(),
            manifest.exercise.id
        );
        let status = progress.get_status(&full_id);

        Ok(Exercise {
            manifest,
            path: path.to_path_buf(),
            status,
            readme_path: path.join("README.md"),
            starter_path: path.join("starter"),
            grader_path: path.join("grader"),
            fixtures_path: if path.join("fixtures").exists() {
                Some(path.join("fixtures"))
            } else {
                None
            },
        })
    }

    /// Get a specific exercise by ID.
    pub fn get_exercise(&self, exercise_id: &str) -> Result<Exercise> {
        // Parse the exercise ID (format: track/id)
        let parts: Vec<&str> = exercise_id.split('/').collect();
        if parts.len() != 2 {
            return Err(Error::Exercise(ExerciseError::NotFound(
                exercise_id.to_string(),
            )));
        }

        let track_name = parts[0];
        let id = parts[1];

        let exercise_path = self.exercises_dir.join(track_name).join(id);
        if !exercise_path.exists() {
            return Err(Error::Exercise(ExerciseError::NotFound(
                exercise_id.to_string(),
            )));
        }

        self.load_exercise(&exercise_path)
    }

    /// Get the current exercise from progress.
    pub fn get_current_exercise(&self) -> Result<String> {
        let progress = load_progress().unwrap_or_default();

        if let Some(current) = progress.current_exercise {
            return Ok(current);
        }

        // Find first incomplete exercise
        let exercises = self.discover_exercises()?;
        let completed = progress.completed_exercises();

        for exercise in exercises {
            let id = exercise.full_id();
            if !completed.contains(&id) && exercise.prerequisites_met(&completed) {
                return Ok(id);
            }
        }

        Err(Error::Exercise(ExerciseError::NotFound(
            "No pending exercises".to_string(),
        )))
    }

    /// Run an exercise and return the result.
    ///
    /// If the exercise is configured for multi-run reliability (`run.runs > 1`),
    /// it will be executed multiple times and the results aggregated.
    pub async fn run_exercise(&self, exercise_id: &str, verbose: bool) -> Result<RunResult> {
        let exercise = self.get_exercise(exercise_id)?;
        let num_runs = exercise.manifest.run.runs;

        // For multi-run exercises, run multiple times and aggregate
        if num_runs > 1 {
            return self
                .run_exercise_multi(exercise_id, &exercise, num_runs, verbose)
                .await;
        }

        self.run_exercise_single(&exercise, verbose).await
    }

    /// Run an exercise multiple times for reliability testing.
    async fn run_exercise_multi(
        &self,
        exercise_id: &str,
        exercise: &Exercise,
        num_runs: u32,
        verbose: bool,
    ) -> Result<RunResult> {
        let start = Instant::now();
        let required_passes = exercise
            .manifest
            .run
            .required_passes
            .unwrap_or(num_runs.div_ceil(2)); // Default: majority must pass

        let mut passed_runs = 0u32;
        let mut total_cost = 0.0f64;
        let mut total_tokens_in = 0u32;
        let mut total_tokens_out = 0u32;
        let mut total_tool_calls = 0u32;
        let mut trace_ids: Vec<String> = Vec::new();

        if verbose {
            println!(
                "Running {} times (need {} to pass)...",
                num_runs, required_passes
            );
        }

        for run_idx in 0..num_runs {
            if verbose {
                println!("  Run {}/{}...", run_idx + 1, num_runs);
            }

            match self.run_exercise_single(exercise, false).await {
                Ok(result) => {
                    if result.passed {
                        passed_runs += 1;
                    }
                    total_cost += result.cost_usd;
                    total_tokens_in += result.tokens_in;
                    total_tokens_out += result.tokens_out;
                    total_tool_calls += result.tool_calls;
                    if let Some(trace_id) = result.trace_id {
                        trace_ids.push(trace_id);
                    }
                }
                Err(_) => {
                    // Run failed with error - counted as a failed run
                }
            }

            // Early exit if already passed threshold
            if passed_runs >= required_passes {
                break;
            }

            // Early exit if impossible to reach threshold
            let remaining_runs = num_runs - run_idx - 1;
            if passed_runs + remaining_runs < required_passes {
                break;
            }
        }

        let duration_secs = start.elapsed().as_secs_f64();
        let passed = passed_runs >= required_passes;

        // Update progress for multi-run
        let mut progress = load_progress().unwrap_or_default();
        let exercise_progress = progress
            .exercises
            .entry(exercise_id.to_string())
            .or_insert_with(|| ExerciseProgress {
                status: ExerciseStatus::InProgress,
                attempts: 0,
                successful_runs: 0,
                total_runs: 0,
                last_attempt: None,
                total_tokens: 0,
                total_cost: 0.0,
            });

        exercise_progress.attempts += 1;
        exercise_progress.total_runs += num_runs;
        exercise_progress.successful_runs += passed_runs;
        exercise_progress.total_tokens += (total_tokens_in + total_tokens_out) as u64;
        exercise_progress.total_cost += total_cost;
        exercise_progress.last_attempt = Some(chrono::Utc::now().to_rfc3339());

        if passed {
            exercise_progress.status = ExerciseStatus::Completed;
        } else if passed_runs > 0 {
            exercise_progress.status = ExerciseStatus::Flaky;
        }

        save_progress(&progress)?;

        Ok(RunResult {
            passed,
            error_message: if passed {
                None
            } else {
                Some(format!(
                    "Reliability threshold not met: {}/{} runs passed, need {}",
                    passed_runs, num_runs, required_passes
                ))
            },
            duration_secs,
            cost_usd: total_cost,
            tool_calls: total_tool_calls,
            tokens_in: total_tokens_in,
            tokens_out: total_tokens_out,
            grading_details: Some(format!(
                "Multi-run reliability: {}/{} passed (required: {})",
                passed_runs, num_runs, required_passes
            )),
            trace_id: trace_ids.first().cloned(),
        })
    }

    /// Run an exercise once and return the result.
    ///
    /// For multi-run exercises, use `run_exercise_multi` instead which calls this
    /// method multiple times and aggregates results.
    ///
    /// When `update_progress` is true, updates the progress file. Set to false
    /// when called from `run_exercise_multi` which handles its own progress tracking.
    async fn run_exercise_single(&self, exercise: &Exercise, _verbose: bool) -> Result<RunResult> {
        let exercise_id = exercise.full_id();
        let start = Instant::now();

        // Load configuration and create provider
        let config = load_or_create_config()?;
        let provider = create_provider(&config)?;

        // Read the exercise prompt from README
        let readme_content = if exercise.readme_path.exists() {
            std::fs::read_to_string(&exercise.readme_path)?
        } else {
            String::new()
        };

        // Read starter content if exists
        let starter_content = if exercise.starter_path.exists() {
            let mut content = String::new();
            for entry in std::fs::read_dir(&exercise.starter_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    content.push_str(&format!("--- {} ---\n", path.display()));
                    content.push_str(&std::fs::read_to_string(&path)?);
                    content.push('\n');
                }
            }
            content
        } else {
            String::new()
        };

        // Build the prompt
        let system_prompt = format!(
            "You are completing an exercise in vibelings, a learning system for agentic programming.\n\n\
            Exercise: {}\n\
            Track: {}\n\n\
            Instructions:\n{}\n\n\
            Starter Content:\n{}",
            exercise.manifest.exercise.title,
            exercise.manifest.exercise.track.display_name(),
            readme_content,
            starter_content
        );

        // Create completion request
        use crate::provider::{CompletionRequest, Message};

        let mut request = CompletionRequest::new(
            &config.model.model,
            vec![
                Message::system(&system_prompt),
                Message::user("Complete the exercise according to the instructions."),
            ],
        );

        if exercise.manifest.requirements.json_mode {
            request = request.with_json_mode();
        }

        request = request.with_temperature(config.model.temperature);

        if let Some(max_tokens) = config.model.max_tokens {
            request = request.with_max_tokens(max_tokens);
        }

        // Execute the request
        let response = provider.complete(request).await?;

        let duration_secs = start.elapsed().as_secs_f64();
        let usage = response.usage();

        // Count tool calls from response
        let tool_call_count = response.tool_calls().map(|tc| tc.len() as u32).unwrap_or(0);

        // Grade the result
        let output = response.text().unwrap_or("");
        let grading_result = self.grader.grade(exercise, output)?;

        // Create trace
        let trace_id = self.trace_store.save_trace(
            &exercise_id,
            &system_prompt,
            output,
            grading_result.passed,
            duration_secs,
        )?;

        // Calculate cost (approximate)
        let cost_usd = usage.estimate_cost(0.003, 0.015); // Claude Sonnet pricing estimate

        // Note: For single-run exercises (runs == 1), progress is tracked here.
        // For multi-run exercises, run_exercise_multi handles progress tracking.
        if exercise.manifest.run.runs == 1 {
            let mut progress = load_progress().unwrap_or_default();
            let exercise_progress = progress
                .exercises
                .entry(exercise_id.clone())
                .or_insert_with(|| ExerciseProgress {
                    status: ExerciseStatus::InProgress,
                    attempts: 0,
                    successful_runs: 0,
                    total_runs: 0,
                    last_attempt: None,
                    total_tokens: 0,
                    total_cost: 0.0,
                });

            exercise_progress.attempts += 1;
            exercise_progress.total_runs += 1;
            exercise_progress.total_tokens += usage.total_tokens as u64;
            exercise_progress.total_cost += cost_usd;
            exercise_progress.last_attempt = Some(chrono::Utc::now().to_rfc3339());

            if grading_result.passed {
                exercise_progress.successful_runs += 1;
                exercise_progress.status = ExerciseStatus::Completed;
            }

            save_progress(&progress)?;
        }

        Ok(RunResult {
            passed: grading_result.passed,
            error_message: if grading_result.passed {
                None
            } else {
                Some(grading_result.message.clone())
            },
            duration_secs,
            cost_usd,
            tool_calls: tool_call_count,
            tokens_in: usage.prompt_tokens,
            tokens_out: usage.completion_tokens,
            grading_details: Some(grading_result.message),
            trace_id: Some(trace_id),
        })
    }

    /// Get hints for an exercise.
    pub fn get_hints(&self, exercise_id: &str) -> Result<Vec<String>> {
        let exercise = self.get_exercise(exercise_id)?;

        // First try to read hints from a hints file
        let hints_path = exercise.path.join("hints.md");
        if hints_path.exists() {
            let content = std::fs::read_to_string(&hints_path)?;
            let hints: Vec<String> = content
                .split("---")
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            return Ok(hints);
        }

        // Otherwise return some generic hints
        Ok(vec![
            "Read the README carefully for requirements.".to_string(),
            "Check the grader schema for expected output format.".to_string(),
            "Look at the starter files for guidance.".to_string(),
        ])
    }

    /// Reset an exercise to its starter state.
    pub fn reset_exercise(&self, exercise_id: &str) -> Result<()> {
        let _exercise = self.get_exercise(exercise_id)?;

        // Reset by removing progress for this exercise
        let mut progress = load_progress().unwrap_or_default();
        progress.exercises.remove(exercise_id);
        save_progress(&progress)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runner_creation() {
        // Should work even without exercises directory
        let runner = ExerciseRunner::new();
        assert!(runner.is_ok());
    }
}
