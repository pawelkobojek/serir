use serir::{error::SerirError, run};
use tokio::signal;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "serir")]
struct Opt {
    /// Port to listen on.
    #[structopt(short, long, default_value = "6379")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), SerirError> {
    let opt = Opt::from_args();
    run(opt.port, signal::ctrl_c()).await
}
