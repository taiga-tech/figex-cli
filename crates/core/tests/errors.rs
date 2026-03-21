use figex_cli_core::{FigexError, PipelineError, RuntimeError};

#[test]
fn figex_error_exit_codes_match_the_spec() {
    let cases = [
        (FigexError::ConnectionFailed(RuntimeError::ProbeFailed), 10),
        (FigexError::TargetNotFound(RuntimeError::TargetNotFound), 11),
        (FigexError::SnapshotFailed(RuntimeError::SnapshotFailed), 12),
        (FigexError::FrameNotFound, 20),
        (
            FigexError::DataInconsistency(PipelineError::FeatureExtractionFailed),
            21,
        ),
        (
            FigexError::NormalizeFailed(PipelineError::NormalizeFailed),
            30,
        ),
        (
            FigexError::ClassifyFailed(PipelineError::ClassificationFailed),
            31,
        ),
        (FigexError::WriteFailed(PipelineError::WriteFailed), 40),
        (FigexError::ConfigInvalid("bad config".to_string()), 50),
    ];

    for (error, expected_code) in cases {
        assert_eq!(error.exit_code(), expected_code);
    }
}
