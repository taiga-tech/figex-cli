/// Version of the ping script.  Bump when the script logic changes.
pub const PING_SCRIPT_VERSION: &str = "1.0.0";

/// Version of the snapshot script.  Bump when the script logic changes.
pub const SNAPSHOT_SCRIPT_VERSION: &str = "1.0.0";

/// Returns a JavaScript expression that confirms the runtime is responsive.
///
/// Evaluating this via `Runtime.evaluate` returns `{ ok: true, ts: <epoch ms> }`.
pub fn ping_script() -> &'static str {
    "(function(){ return { ok: true, ts: Date.now() }; })()"
}

/// Returns a JavaScript expression used for the first-stage snapshot check.
///
/// In the first implementation stage this simply confirms that JS evaluation
/// succeeds.  Actual Figma frame data extraction is deferred to stage 2.
pub fn snapshot_script() -> &'static str {
    "(function(){ return { ok: true, ts: Date.now() }; })()"
}
