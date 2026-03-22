use std::time::Duration;

use figex_cli_core::RuntimeError;

use super::protocol::JsonTarget;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

pub struct ProbeConfig {
    /// Explicit host override (e.g. from CLI `--host`).
    pub host: Option<String>,
    /// Explicit port override (e.g. from CLI `--port`).
    pub port: Option<u16>,
    pub timeout: Duration,
}

// ---------------------------------------------------------------------------
// Result type
// ---------------------------------------------------------------------------

pub struct DiscoveredTarget {
    pub host: String,
    pub port: u16,
    pub target_id: String,
    pub title: String,
    pub url: String,
    pub ws_debugger_url: String,
    pub score: u32,
}

// ---------------------------------------------------------------------------
// Scoring
// ---------------------------------------------------------------------------

/// Score a `/json/list` entry — higher means more likely to be Figma.
fn score_target(t: &JsonTarget) -> u32 {
    if t.websocket_debugger_url.is_none() {
        return 0;
    }
    let mut score = 0u32;
    if t.kind == "page" {
        score += 2;
    }
    let title_lc = t.title.to_lowercase();
    let url_lc = t.url.to_lowercase();
    if title_lc.contains("figma") {
        score += 3;
    }
    if url_lc.contains("figma") {
        score += 3;
    }
    // Exclude well-known false positives (DevTools pages, extensions, etc.)
    let devtools_keywords = ["devtools", "chrome-extension", "chrome://", "about:blank"];
    if devtools_keywords.iter().any(|kw| url_lc.contains(kw)) {
        return 0;
    }
    score
}

// ---------------------------------------------------------------------------
// Port candidate list
// ---------------------------------------------------------------------------

/// Known CDP ports to probe before resorting to a full range scan.
const KNOWN_CDP_PORTS: &[u16] = &[9229, 9222];

/// MCP desktop server default port.
const MCP_PORT: u16 = 3845;

/// Returns the ordered list of (host, port) candidates to probe.
///
/// Discovery order per docs:
///   1. Explicit host/port from CLI or config
///   2. Known CDP port candidates
///   3. 9222–9322 range scan
///   4. MCP default endpoint (127.0.0.1:3845)
#[doc(hidden)]
pub fn port_candidates(config: &ProbeConfig) -> Vec<(String, u16)> {
    let host = config
        .host
        .clone()
        .unwrap_or_else(|| "127.0.0.1".to_string());

    // If the user supplied an explicit port, only try that one.
    if let Some(port) = config.port {
        return vec![(host, port)];
    }

    let mut candidates: Vec<(String, u16)> = Vec::new();

    // Known CDP ports first.
    for &port in KNOWN_CDP_PORTS {
        candidates.push((host.clone(), port));
    }

    // Full range scan (skip ports already in KNOWN_CDP_PORTS).
    for port in 9222u16..=9322 {
        if !KNOWN_CDP_PORTS.contains(&port) {
            candidates.push((host.clone(), port));
        }
    }

    // MCP desktop endpoint last.
    candidates.push(("127.0.0.1".to_string(), MCP_PORT));

    candidates
}

// ---------------------------------------------------------------------------
// HTTP fetch helpers
// ---------------------------------------------------------------------------

/// Probe `GET /json/version` to confirm the port speaks CDP.
///
/// Returns `true` when the endpoint is reachable and returns a valid JSON
/// object.  The caller should skip the port when this returns `false`.
async fn probe_version(host: &str, port: u16, client: &reqwest::Client) -> bool {
    let url = format!("http://{}:{}/json/version", host, port);
    client
        .get(&url)
        .send()
        .await
        .ok()
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

async fn fetch_targets(host: &str, port: u16, client: &reqwest::Client) -> Option<Vec<JsonTarget>> {
    let url = format!("http://{}:{}/json/list", host, port);
    client
        .get(&url)
        .send()
        .await
        .ok()?
        .json::<Vec<JsonTarget>>()
        .await
        .ok()
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Probe all candidate (host, port) pairs and return the best Figma target.
///
/// Returns `RuntimeError::TargetNotFound` when no matching target is found.
pub async fn discover_best_target(config: &ProbeConfig) -> Result<DiscoveredTarget, RuntimeError> {
    let http = reqwest::Client::builder()
        .timeout(config.timeout)
        .build()
        .expect("reqwest client construction should not fail for probe discovery");

    let candidates = port_candidates(config);

    for (host, port) in candidates {
        // Guard: confirm this port speaks CDP before fetching the full target list.
        if !probe_version(&host, port, &http).await {
            continue;
        }

        let Some(targets) = fetch_targets(&host, port, &http).await else {
            continue;
        };

        let best = targets
            .iter()
            .filter_map(|t| {
                let s = score_target(t);
                if s > 0 {
                    Some((t, s))
                } else {
                    None
                }
            })
            .max_by_key(|(_, s)| *s);

        if let Some((t, score)) = best {
            return Ok(DiscoveredTarget {
                host,
                port,
                target_id: t.id.clone(),
                title: t.title.clone(),
                url: t.url.clone(),
                ws_debugger_url: t
                    .websocket_debugger_url
                    .clone()
                    .expect("target with score > 0 always has a websocket_debugger_url"),
                score,
            });
        }
    }

    Err(RuntimeError::TargetNotFound)
}
