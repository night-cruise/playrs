#![allow(non_snake_case)]
use cli::Opt;

use serde::{Deserialize, Serialize};

pub mod cli;

const REQUEST_URL: &str = "https://play.rust-lang.org/execute";

pub fn run(args: Opt) -> Result<(), Box<dyn std::error::Error>> {
    let _ = args.validate_args()?;
    let request_body = args.build_request_body()?;
    let response_body = send_request(request_body)?;
    output_response(response_body);

    Ok(())
}

#[derive(Debug, Serialize)]
struct RequestBody {
    code: String,
    channel: String,
    edition: String,
    mode: String,
    crateType: String,
    tests: bool,
    backtrace: bool,
}

#[derive(Debug, Deserialize)]
struct ResponseBody {
    success: bool,
    stderr: String,
    stdout: String,
}

fn send_request(request_body: RequestBody) -> reqwest::Result<ResponseBody> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(REQUEST_URL)
        .json(&request_body)
        .send()?
        .json::<ResponseBody>()?;

    Ok(resp)
}

fn output_response(response_body: ResponseBody) {
    let ResponseBody {
        success,
        stderr,
        stdout,
    } = response_body;

    println!(
        "\n----------------------------------Standard Error----------------------------------\n"
    );
    for line in stderr.lines() {
        println!("{}", line);
    }
    if success {
        println!("\n----------------------------------Standard Output----------------------------------\n");
        for line in stdout.lines() {
            println!("{}", line);
        }
    }
}
