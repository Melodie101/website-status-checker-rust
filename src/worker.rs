use crate::{checker, config::Config};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};



pub fn start_worker_pool(config: &Config) -> Vec<checker::WebsiteStatus> {
    let (sender, receiver): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    let results = Arc::new(Mutex::new(vec![]));
    let receiver = Arc::new(Mutex::new(receiver));
    let mut handles = vec![];

    for _ in 0..config.workers {
        let rx = Arc::clone(&receiver);
        let results = Arc::clone(&results);
        let timeout = config.timeout;
        let retries = config.retries;

        let handle = thread::spawn(move || {
            loop {
                let url = {
                    let lock = rx.lock().unwrap();
                    lock.recv()
                };
                match url {
                    Ok(u) => {
                        let status = checker::check_website(&u, timeout, retries);
                println!(
                    "{} => {} in {}ms",
                    u,
                    status
                        .action_status
                        .as_ref()
                        .map(|c| c.to_string())
                        .unwrap_or_else(|e| e.clone()),
                    status.response_time.as_millis()
                );
                results.lock().unwrap().push(status);
                    }
                    Err(_) => break,
                }
            }

    });

   handles.push(handle);
    }

    for url in &config.urls {
        sender.send(url.clone()).unwrap();
    }
    drop(sender);

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}
