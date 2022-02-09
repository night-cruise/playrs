use playrs::{Result, StructOpt};

fn main() -> Result<()> {
    let args = playrs::cli::Opt::from_args();

    playrs::run(args)?;

    Ok(())
}
