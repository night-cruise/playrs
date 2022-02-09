use crate::{Deserialize, Serialize};

const REQUEST_URL: &str = "https://play.rust-lang.org/execute";

#[derive(Debug, Serialize)]
pub(crate) struct RequestBody {
    pub(crate) code: String,
    pub(crate) channel: String,
    pub(crate) edition: String,
    pub(crate) mode: String,
    pub(crate) crateType: String,
    pub(crate) tests: bool,
    pub(crate) backtrace: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ResponseBody {
    success: bool,
    stderr: String,
    stdout: String,
}

pub(crate) fn send_request(request_body: RequestBody) -> reqwest::Result<ResponseBody> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(REQUEST_URL)
        .json(&request_body)
        .send()?
        .json::<ResponseBody>()?;

    Ok(resp)
}

pub(crate) fn output_response(response_body: ResponseBody) {
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
