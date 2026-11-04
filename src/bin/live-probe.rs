//! Live-API probe: exercise the real Ferro server from a native client
//! (the exact calls the fresco app would make). Feature-gated because it
//! needs a reachable server.
//!
//! ```text
//! cargo run --features live -- http://localhost:8199 admin 'PASSWORD'
//! ```

use base64::Engine as _;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let base = args.get(1).map(|s| s.trim_end_matches('/').to_string()).unwrap_or_else(|| "http://localhost:8199".into());
    let user = args.get(2).cloned().unwrap_or_else(|| "admin".into());
    let pass = args.get(3).cloned().unwrap_or_else(|| "Drive-T3st!pass".into());
    let creds = base64::engine::general_purpose::STANDARD.encode(format!("{user}:{pass}"));
    let auth = format!("Basic {creds}");
    let client = reqwest::blocking::Client::builder().timeout(std::time::Duration::from_secs(20)).build().expect("client");

    // 1. auth + listing
    let t = Instant::now();
    let resp = client
        .get(format!("{base}/api/v1/files?path=/"))
        .header("Authorization", &auth)
        .send()
        .expect("list request");
    let list_ms = t.elapsed().as_millis();
    let entries: Vec<String> = resp
        .json::<serde_json::Value>()
        .ok()
        .and_then(|d| {
            d.get("entries").map(|e| {
                e.as_array()
                    .map(|a| a.iter().filter_map(|x| x.get("name").and_then(|n| n.as_str()).map(String::from)).collect())
                    .unwrap_or_default()
            })
        })
        .unwrap_or_default();
    println!("list: {} ms | entries: {entries:?}", list_ms);

    // 2. share video: ranged read through the proxy chain
    let token = args.get(4).cloned().unwrap_or_else(|| "940c76fa-eaad-46db-9528-4b34a225be10".into());
    let t = Instant::now();
    let resp = client
        .get(format!("{base}/s/{token}"))
        .header("Range", "bytes=1000000-1000063")
        .send()
        .expect("share request");
    let share_ms = t.elapsed().as_millis();
    let status = resp.status().as_u16();
    let ctype = resp.headers().get("content-type").map(|v| v.to_str().unwrap_or("?").to_string()).unwrap_or_default();
    let crange = resp.headers().get("content-range").map(|v| v.to_str().unwrap_or("?").to_string()).unwrap_or_default();
    let len = resp.content_length().unwrap_or(0);
    println!("video: {share_ms} ms | {status} {ctype} {crange} | {len} B");

    // 3. upload: PUT a small file through DAV
    let t = Instant::now();
    let resp = client
        .put(format!("{base}/dav/spike/probe-{}.txt", std::process::id()))
        .header("Authorization", &auth)
        .body("ferro fresco spike upload probe".as_bytes().to_vec())
        .send()
        .expect("upload request");
    let up_ms = t.elapsed().as_millis();
    println!("upload: {up_ms} ms | {}", resp.status().as_u16());
}
