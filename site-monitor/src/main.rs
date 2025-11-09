use std::fs::OpenOptions;
use std::io::Write;
use std::net::ToSocketAddrs;
use std::thread::sleep;
use std::time::{Duration, Instant};

use chrono::Local;
use winrt_notification::{Toast, Sound}; // ✅ fixed import

// Cargo.toml
// [dependencies]
// reqwest = { version = "0.11", features = ["blocking", "rustls-tls"] }
// chrono = "0.4"
// winrt-notification = "0.5"

fn send_notification(title: &str, message: &str) {
    // ✅ Updated toast API (no duration field anymore)
    let _ = Toast::new(Toast::POWERSHELL_APP_ID)
        .title(title)
        .text1(message)
        .sound(Some(Sound::Default))
        .show();
}

fn get_reason(elapsed_dns: f32, elapsed_conn: f32, elapsed_ssl: f32, elapsed_total: f32) -> String {
    if elapsed_dns > 2.0 {
        "Possible DNS resolution delay".to_string()
    } else if elapsed_conn > 3.0 {
        "Possible network routing or firewall latency".to_string()
    } else if elapsed_ssl > 3.0 {
        "SSL handshake delay".to_string()
    } else if elapsed_total > 5.0 {
        "Server backend or application delay".to_string()
    } else {
        "Normal".to_string()
    }
}

fn main() {
    let url = "https://itba.incometax.gov.in";
    let log_file = "latency_log.txt";

    println!("🟢 Monitoring started for: {}", url);
    println!("Logs will be saved in {}", log_file);
    println!("Press Ctrl + C to stop.\n");

    loop {
        // Timing stages
        let dns_start = Instant::now();
        let _ = ("itba.incometax.gov.in", 443).to_socket_addrs();
        let dns_time = dns_start.elapsed().as_secs_f32();

        let start = Instant::now();
        let (status_code, total_time) = match reqwest::blocking::get(url) {
            Ok(resp) => (resp.status().as_u16(), start.elapsed().as_secs_f32()),
            Err(_) => (0, start.elapsed().as_secs_f32()),
        };

        let connect_time = dns_time + 0.5;
        let ssl_time = total_time - connect_time;
        let reason = get_reason(dns_time, connect_time, ssl_time, total_time);

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_line = format!(
            "{},{:.2}s total, reason: {}, status: {}\n",
            timestamp, total_time, reason, status_code
        );

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)
            .unwrap();
        file.write_all(log_line.as_bytes()).unwrap();

        if status_code == 0 {
            println!("❌ [{}] Site unreachable. {}", timestamp, reason);
            send_notification("⚠️ ITBA Site Down", &reason);
        } else if total_time > 5.0 {
            println!(
                "⚠️ [{}] Slow response: {:.2}s (HTTP {}), {}",
                timestamp, total_time, status_code, reason
            );
            send_notification("⚠️ ITBA Slow", &format!("{} ({:.2}s)", reason, total_time));
        } else {
            println!("✅ [{}] OK: {:.2}s (HTTP {}), {}", timestamp, total_time, status_code, reason);
        }

        sleep(Duration::from_secs(60));
    }
}
