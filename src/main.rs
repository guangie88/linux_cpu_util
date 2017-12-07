#![feature(slice_patterns)]

use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

fn loop_impl(
    prev_idle: u64,
    prev_total: u64,
) -> Result<(u64, u64), Box<Error>> {
    let file = File::open("/proc/stat")?;
    let mut line = String::new();
    BufReader::new(&file).read_line(&mut line)?;

    let tokens: Vec<&str> = line.split_whitespace().collect();

    match *tokens.as_slice() {
        [_, user, nice, system, idle, _..] => {
            let [user, nice, system, idle] = [
                user.parse::<u64>()?,
                nice.parse::<u64>()?,
                system.parse::<u64>()?,
                idle.parse::<u64>()?,
            ];

            let total = user + nice + system + idle;
            let idle_delta = idle - prev_idle;
            let total_delta = total - prev_total;

            let util = 100.0 * (1.0 - idle_delta as f64 / total_delta as f64);

            println!("CPU usage: {:.3}%", util);
            Ok((idle, total))
        }
        _ => Err("Invalid /proc/stat result")?,
    }
}

fn run() -> Result<(), Box<Error>> {
    let args = args().collect::<Vec<_>>();

    if let [_, ref interval_sec] = *args.as_slice() {
        let interval_sec: u64 = interval_sec.parse()?;
        let interval = Duration::from_secs(interval_sec);

        let loop_vals = (0..)
            .scan(
                (0u64, 0u64),
                |&mut (ref mut prev_idle, ref mut prev_total), _| {
                    match loop_impl(*prev_idle, *prev_total) {
                        Ok((idle, total)) => {
                            *prev_idle = idle;
                            *prev_total = total;
                            Some(())
                        }
                        Err(e) => {
                            eprintln!("Loop ERROR: {}", e);
                            None
                        }
                    }
                },
            )
            .inspect(|_| sleep(interval));

        for _ in loop_vals {}
    } else {
        println!("Usage: program <interval sec>");
    }

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {
            exit(0);
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            exit(1);
        }
    }
}
