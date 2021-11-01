use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(parse(from_os_str))]
    payload_path: PathBuf,
}

fn main() {
    let opts = Opts::from_args();
    println!("Doubling payload @ {:?}", opts.payload_path);
}
