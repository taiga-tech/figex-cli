use std::time::Duration;

use figex_cli_runtime::transport::cdp::probe::port_candidates;
use figex_cli_runtime::ProbeConfig;

#[test]
fn port_candidates_respects_explicit_port() {
    let config = ProbeConfig {
        host: Some("localhost".to_string()),
        port: Some(9555),
        timeout: Duration::from_secs(1),
    };

    assert_eq!(
        port_candidates(&config),
        vec![("localhost".to_string(), 9555)]
    );
}

#[test]
fn port_candidates_includes_known_ports_range_scan_and_mcp_fallback() {
    let config = ProbeConfig {
        host: Some("example.local".to_string()),
        port: None,
        timeout: Duration::from_secs(1),
    };

    let candidates = port_candidates(&config);
    let unique_range_ports = (9222u16..=9322)
        .filter(|port| !matches!(port, 9222 | 9229))
        .count();
    let expected_len = 2 + unique_range_ports + 1;

    assert_eq!(
        candidates.first(),
        Some(&("example.local".to_string(), 9229))
    );
    assert_eq!(
        candidates.get(1),
        Some(&("example.local".to_string(), 9222))
    );
    assert!(candidates.contains(&("example.local".to_string(), 9223)));
    assert!(candidates.contains(&("example.local".to_string(), 9224)));
    assert!(candidates.contains(&("example.local".to_string(), 9322)));
    assert_eq!(candidates.last(), Some(&("127.0.0.1".to_string(), 3845)));
    assert_eq!(candidates.len(), expected_len);
}

#[test]
fn port_candidates_default_host_is_loopback() {
    let config = ProbeConfig {
        host: None,
        port: Some(9222),
        timeout: Duration::from_secs(1),
    };

    assert_eq!(
        port_candidates(&config),
        vec![("127.0.0.1".to_string(), 9222)]
    );
}

#[test]
fn port_candidates_without_host_use_loopback_across_full_scan() {
    let config = ProbeConfig {
        host: None,
        port: None,
        timeout: Duration::from_secs(1),
    };

    let candidates = port_candidates(&config);

    assert_eq!(candidates.first(), Some(&("127.0.0.1".to_string(), 9229)));
    assert_eq!(candidates.get(1), Some(&("127.0.0.1".to_string(), 9222)));
    assert!(candidates.contains(&("127.0.0.1".to_string(), 9223)));
    assert!(candidates.contains(&("127.0.0.1".to_string(), 9322)));
    assert_eq!(candidates.last(), Some(&("127.0.0.1".to_string(), 3845)));
}
