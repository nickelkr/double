use std::path::PathBuf;
use structopt::StructOpt;
use warp;

mod endpoint;
use endpoint::Endpoint;

mod router;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long, default_value = "/")]
    path: String,

    #[structopt(parse(from_os_str))]
    payload_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let opts = Opts::from_args();

    let endpoint = match Endpoint::from_file(opts.path, opts.payload_path) {
        Ok(endpoint) => endpoint,
        Err(e) => return Err(e),
    };
    let routes = router::router(endpoint);

    warp::serve(routes).run(([127, 0, 0, 1], 3131)).await;
    Ok(())
}
