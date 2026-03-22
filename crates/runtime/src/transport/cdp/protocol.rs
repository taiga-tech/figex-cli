use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// CDP wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct CdpRequest {
    pub id: u32,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct CdpResponse {
    pub id: Option<u32>,
    pub result: Option<serde_json::Value>,
    pub error: Option<CdpError>,
}

#[derive(Debug, Deserialize)]
pub struct CdpError {
    pub code: i32,
    pub message: String,
}

// ---------------------------------------------------------------------------
// /json/list target descriptor
// ---------------------------------------------------------------------------

/// One entry returned by `GET /json/list`.
#[derive(Debug, Deserialize)]
pub struct JsonTarget {
    pub id: String,
    pub title: String,
    pub url: String,
    #[serde(rename = "type")]
    pub kind: String,
    // CDP uses `webSocketDebuggerUrl` (capital S) — explicit rename required.
    #[serde(rename = "webSocketDebuggerUrl")]
    pub websocket_debugger_url: Option<String>,
}
