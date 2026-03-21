pub fn greet() -> String {
    "Hello from figex-cli-core!".to_string()
}

// ---------------------------------------------------------------------------
// Error taxonomy
// ---------------------------------------------------------------------------

/// Errors related to runtime (Figma Desktop) operations.
#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("probe failed")]
    ProbeFailed,
    #[error("target not found")]
    TargetNotFound,
    #[error("attach failed")]
    AttachFailed,
    #[error("timed out")]
    Timeout,
    #[error("evaluate failed")]
    EvaluateFailed,
    #[error("snapshot failed")]
    SnapshotFailed,
}

/// Errors related to the extract→normalize→report pipeline.
#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("raw mapping failed")]
    RawMappingFailed,
    #[error("feature extraction failed")]
    FeatureExtractionFailed,
    #[error("normalize failed")]
    NormalizeFailed,
    #[error("classification failed")]
    ClassificationFailed,
    #[error("report failed")]
    ReportFailed,
    #[error("write failed")]
    WriteFailed,
}

/// Top-level error type for figex operations.
///
/// Carry an exit-code so `main` can call `process::exit` with the correct
/// value defined in the architecture spec.
#[derive(Debug, thiserror::Error)]
pub enum FigexError {
    /// exit 10 — cannot connect to runtime
    #[error("connection error: {0}")]
    ConnectionFailed(#[source] RuntimeError),

    /// exit 11 — target process not found
    #[error("target not found: {0}")]
    TargetNotFound(#[source] RuntimeError),

    /// exit 12 — snapshot acquisition failed
    #[error("snapshot error: {0}")]
    SnapshotFailed(#[source] RuntimeError),

    /// exit 20 — specified frame does not exist
    #[error("frame not found")]
    FrameNotFound,

    /// exit 21 — data inconsistency
    #[error("data inconsistency: {0}")]
    DataInconsistency(#[source] PipelineError),

    /// exit 30 — normalizer failed
    #[error("normalize error: {0}")]
    NormalizeFailed(#[source] PipelineError),

    /// exit 31 — classifier failed
    #[error("classify error: {0}")]
    ClassifyFailed(#[source] PipelineError),

    /// exit 40 — file write failed
    #[error("write error: {0}")]
    WriteFailed(#[source] PipelineError),

    /// exit 50 — invalid configuration
    #[error("config error: {0}")]
    ConfigInvalid(String),
}

impl FigexError {
    /// Map error variant to the exit code defined in the architecture spec.
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::ConnectionFailed(_) => 10,
            Self::TargetNotFound(_) => 11,
            Self::SnapshotFailed(_) => 12,
            Self::FrameNotFound => 20,
            Self::DataInconsistency(_) => 21,
            Self::NormalizeFailed(_) => 30,
            Self::ClassifyFailed(_) => 31,
            Self::WriteFailed(_) => 40,
            Self::ConfigInvalid(_) => 50,
        }
    }
}
