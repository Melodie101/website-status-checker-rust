
use reqwest::blocking::Client;
use std::time::{Duration, Instant, SystemTime};

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub action_status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: SystemTime,
}

pub fn check_website(url: &str, timeout: u64, retries: usize) -> WebsiteStatus {
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout))
        .build()
        .unwrap();

    let start = Instant::now();
    let mut attempts = 0;

    let result = loop {
        let res = client.get(url).send();
        if res.is_ok() || attempts >= retries {
            break res;
        }
        attempts += 1;
        std::thread::sleep(Duration::from_millis(100));
    };

    let status = result.map(|r| r.status().as_u16()).map_err(|e| e.to_string());

    WebsiteStatus {
        url: url.to_string(),
        action_status: status,
        response_time: start.elapsed(),
        timestamp: SystemTime::now(),
    }
}

pub fn generate_json(results: &[WebsiteStatus]) -> String {
    let mut entries = vec![];
    for r in results {
        let entry = format!(
            r#"{{
    "url": "{}",
    "status": {},
    "response_time_ms": {},
    "timestamp": "{}"
}}"#,
            r.url,
            match &r.action_status {
                Ok(code) => code.to_string(),
                Err(err) => format!(r#""{}""#, err),
            },
            r.response_time.as_millis(),
            r.timestamp
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        entries.push(entry);
    }
    format!("[\n{}\n]", entries.join(",\n"))
}
