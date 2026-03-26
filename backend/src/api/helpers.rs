use axum::http::HeaderMap;
use std::net::SocketAddr;

/// Extract client IP address using ConnectInfo (actual socket address) as the
/// trusted source. Only falls back to proxy headers when a real peer address
/// cannot be determined. Uses `Fly-Client-IP` (Fly.io) first, then
/// `X-Forwarded-For` (last entry = proxy-added), then the socket address.
pub fn extract_client_ip(headers: &HeaderMap) -> String {
    extract_client_ip_with_peer(headers, None)
}

/// Extract client IP with an optional peer socket address for higher trust.
pub fn extract_client_ip_with_peer(headers: &HeaderMap, peer: Option<SocketAddr>) -> String {
    // Fly.io sets this header — it is trustworthy (not client-controlled)
    if let Some(fly_ip) = headers
        .get("fly-client-ip")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
    {
        return fly_ip;
    }

    // Use actual socket address when available (most trustworthy)
    if let Some(addr) = peer {
        return addr.ip().to_string();
    }

    // Fallback: X-Forwarded-For (take last entry — the one added by the trusted proxy)
    if let Some(xff) = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
    {
        if let Some(last) = xff.rsplit(',').next() {
            let trimmed = last.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }

    headers
        .get("x-real-ip")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
