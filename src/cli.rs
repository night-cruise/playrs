#![allow(unused)]
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::RequestBody;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Play Rust", about = "Compile and run your rust code")]
pub struct Opt {
    /// rust code file
    #[structopt(parse(from_os_str))]
    file: PathBuf,

    /// Compile channel: stable, nightly or beta
    #[structopt(short, long, default_value = "stable")]
    channel: String,

    /// Compile edition: 2015, 2018 or 2021
    #[structopt(short, long, default_value = "2021")]
    edition: String,

    /// Compile mode: debug or release
    #[structopt(short, long, default_value = "debug")]
    mode: String,

    /// Crate type: bin or lib
    #[structopt(short = "p", long, default_value = "bin")]
    program_type: String,

    /// Whether it is a test
    #[structopt(short, long)]
    tests: bool,

    /// Whether to enable backtrace
    #[structopt(short, long)]
    backtrace: bool,
}

impl Opt {
    pub(crate) fn build_request_body(self) -> io::Result<RequestBody> {
        let Opt {
            file,
            channel,
            edition,
            mode,
            program_type,
            tests,
            backtrace,
        } = self;
        let code = fs::read_to_string(file)?;

        Ok(RequestBody {
            code,
            channel,
            edition,
            mode,
            crateType: program_type,
            tests,
            backtrace,
        })
    }
}
