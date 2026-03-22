#[path = "support/mock_cdp_server.rs"]
mod mock_cdp_server;

use std::fmt;
use std::io::{self, Write};

use figex_cli::app::AppContext;
use figex_cli::commands::attach;
use figex_cli::config::Settings;

fn attach_context(host: String, port: u16) -> AppContext {
    AppContext::new(Settings {
        host,
        host_explicit: true,
        port,
        port_explicit: true,
        ..Settings::default()
    })
}

struct FailOnWriteFmtCall {
    calls: usize,
    fail_on: usize,
}

impl FailOnWriteFmtCall {
    fn new(fail_on: usize) -> Self {
        Self { calls: 0, fail_on }
    }
}

impl Write for FailOnWriteFmtCall {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> io::Result<()> {
        self.calls += 1;
        if self.calls == self.fail_on {
            return Err(io::Error::other("write failed"));
        }

        let mut rendered = String::new();
        fmt::write(&mut rendered, args).map_err(|_| io::Error::other("format failed"))?;
        Ok(())
    }
}

async fn spawn_attach_context() -> AppContext {
    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_one(ws_listener));
    let http_port =
        mock_cdp_server::spawn_figma_cdp_server("t-attach-command", "Figma - Attach", ws_port)
            .await;

    attach_context("127.0.0.1".to_string(), http_port)
}

#[tokio::test]
async fn attach_command_writes_session_details() {
    let ctx = spawn_attach_context().await;
    let mut stdout = Vec::new();

    attach::run(&ctx, &mut stdout)
        .await
        .expect("attach should succeed with a mock runtime");

    let output = String::from_utf8(stdout).expect("stdout should be valid UTF-8");
    assert!(output.contains("Attached to Figma runtime"));
    assert!(output.contains("host:      127.0.0.1"));
    assert!(output.contains("target_id: t-attach-command"));
    assert!(output.contains("title:     Figma - Attach"));
}

#[tokio::test]
async fn attach_command_propagates_each_stdout_write_failure() {
    for fail_on in 1..=6 {
        let ctx = spawn_attach_context().await;
        let mut stdout = FailOnWriteFmtCall::new(fail_on);

        let error = attach::run(&ctx, &mut stdout)
            .await
            .expect_err("stdout write failure should bubble up");

        assert!(error
            .downcast_ref::<io::Error>()
            .map(|inner| inner.kind() == io::ErrorKind::Other)
            .unwrap_or(false));
    }
}
