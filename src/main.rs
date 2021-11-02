use std::path::PathBuf;
use structopt::StructOpt;

mod endpoint;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long, default_value = "/")]
    path: String,

    #[structopt(parse(from_os_str))]
    payload_path: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let opts = Opts::from_args();

    let endpoint = match endpoint::Endpoint::from_file(opts.path, opts.payload_path) {
        Ok(endpoint) => endpoint,
        Err(e) => return Err(e),
    };

    println!("Doubling path: {}", endpoint.path);
    println!("Doubling payload: \n{}", endpoint.payload);
    Ok(())
}
