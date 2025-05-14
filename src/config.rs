use std::{env, fs::File, io::BufRead, io::BufReader};

pub struct Config {
    pub urls: Vec<String>,
    pub workers: usize,
    pub timeout: u64,
    pub retries: usize,
}

impl Config {
    pub fn parse() -> Result<Self, String> {
        let args: Vec<String> = env::args().collect();
        let mut urls = Vec::new();
        let mut file = None;
        let mut workers = 4;
        let mut timeout = 5;
        let mut retries = 0;

        let mut i =1;
        while i < args.len() {
            match args[i].as_str() {
                "--file" => {
                    i += 1;
                    if i < args.len() {
                        file = Some(args[i].clone());

                    } else {
                        return Err("Missing path after --file".into());
                    }
                }
                "--workers" => {
                    i += 1;
                    workers = args.get(i).and_then(|s| s.parse::<usize>().ok()).unwrap_or(workers);
                }
                "--timeout" => {
                    i += 1;
                    timeout = args.get(i).and_then(|s| s.parse::<u64>().ok()).unwrap_or(timeout);
                }
                "--retries" => {
                    i += 1;
                    retries = args.get(i).and_then(|s| s.parse::<usize>().ok()).unwrap_or(retries);
                }
                _ if args[i].starts_with("--") => return Err(format!("Unknown flag: {}", args[i])),
                _ => urls.push(args[i].clone()),
            }
            i += 1;
        }

        if let Some(file_path) = file {
            let reader = BufReader::new(File::open(file_path).map_err(|e| e.to_string())?);
            for line in reader.lines() {
                let line = line.map_err(|e| e.to_string())?;
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    urls.push(trimmed.to_string());
                }
            }
        }

        if urls.is_empty() {
            return Err("No URLs provided via --file or arguments.".into());
        }

        Ok(Config {
            urls,
            workers,
            timeout,
            retries,
        })
    }
}