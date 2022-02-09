use crate::{fs, io, ParseArgsError, PathBuf, RequestBody, StructOpt};

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

    pub(crate) fn validate_args(&self) -> Result<(), ParseArgsError> {
        match self.channel.as_str() {
            "stable" | "nightly" | "beta" => {}
            _ => {
                return Err(ParseArgsError::new(
                    "the value of channel can be only stable, nightly or beta",
                ))
            }
        }
        match self.edition.as_str() {
            "2015" | "2018" | "2021" => {}
            _ => {
                return Err(ParseArgsError::new(
                    "the value of edition can be only 2015, 2018 or 2021",
                ))
            }
        }
        match self.mode.as_str() {
            "debug" | "release" => {}
            _ => {
                return Err(ParseArgsError::new(
                    "the value of mode can be only debug or release",
                ))
            }
        }
        match self.program_type.as_str() {
            "bin" | "lib" => {}
            _ => {
                return Err(ParseArgsError::new(
                    "the value of crate type can be only bin or lib",
                ))
            }
        }

        Ok(())
    }
}
