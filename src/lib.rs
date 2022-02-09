#![allow(non_snake_case)]
use cli::Opt;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;
use std::{error, thread};
use std::{fs, io};

pub use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
pub use structopt::StructOpt;

use crate::errors::{GetLockError, ParseArgsError};
use crate::request::RequestBody;
use crate::request::{output_response, send_request};

pub mod cli;
mod errors;
mod request;

pub fn run(args: Opt) -> Result<(), anyhow::Error> {
    let _ = args.validate_args().context("parse args failed")?;
    let request_body = args
        .build_request_body()
        .context("build request body failed")?;

    let done = build_done();
    spinner("waiting", done.clone());

    let response_body = send_request(request_body).context("send request failed")?;
    done_notify_one(done).context("notify spinner thread failed")?;
    output_response(response_body);

    Ok(())
}

fn build_done() -> Arc<(Mutex<bool>, Condvar)> {
    Arc::new((Mutex::new(false), Condvar::new()))
}

fn done_notify_one(done: Arc<(Mutex<bool>, Condvar)>) -> Result<(), GetLockError> {
    let (lock, condvar) = &*done;
    let mut event = lock.lock().map_err(|_| GetLockError)?;
    *event = true;
    condvar.notify_one();

    Ok(())
}

fn spinner(msg: &'static str, done: Arc<(Mutex<bool>, Condvar)>) {
    thread::spawn(move || {
        let (lock, condvar) = &*done;
        let mut event = lock.lock().unwrap();

        let mut stdout = io::stdout();
        let cycle = ['\\', '|', '/', '-'].into_iter().cycle();

        println!();
        for char in cycle {
            print!("\r{} {}...", char, msg);
            stdout.flush().unwrap();

            let result = condvar
                .wait_timeout(event, Duration::from_millis(100))
                .unwrap();
            event = result.0;
            if *event {
                break;
            }
        }
    });
}
