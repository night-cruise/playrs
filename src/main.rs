use structopt::StructOpt;

fn main() {
    let args = playrs::cli::Opt::from_args();
    if let Err(err) = playrs::run(args) {
        eprintln!("Application error: {}", err);
        std::process::exit(1);
    }
}
