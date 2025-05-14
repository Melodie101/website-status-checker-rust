# website-status-checker-rust

## Overall Implementation
This project concurrently checks the status of websites via HTTP and reports their availability, response time, and more.

## Features
- Input Sources: sites.txt

- Concurrency through thread pool (--workers)
- Per-request timeout (--timeout)
- Retries (--retries)
- Live terminal output upon each request
- Output results to 'status.json' containing an array of objects with the same data.

---
## Build Instructions

```sh
## Debug Build:
$ cargo build
## Release Build:
$ cargo build --release
```

## Usage Examples

```sh
## From text file:
$ ./target/release/website_status_checker --file sites.txt

## With additional flags:
$ ./target/release/website_status_checker --file sites.txt --workers 4 --timeout 3 --retries 2

## From direct URLs:
$ ./target/release/website_status_checker https://www.google.com https://example.com

```