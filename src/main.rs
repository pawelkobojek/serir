use serir::run;
use std::error::Error;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "serir")]
struct Opt {
    /// Port to listen on.
    #[structopt(short, long, default_value = "6379")]
    port: u16,

    /// Number of workers in a threadpool
    #[structopt(short, long, default_value = "4")]
    num_workers: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    run(opt.port, opt.num_workers)
}
