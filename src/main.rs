use serir::run;
use std::error::Error;
use tokio::sync::oneshot;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "serir")]
struct Opt {
    /// Port to listen on.
    #[structopt(short, long, default_value = "6379")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let (tx, rx) = oneshot::channel();
    let mut tx = Some(tx);
    ctrlc::set_handler(move || {
        tx.take().unwrap().send(true).unwrap();
    })
    .expect("Error setting ctrl-c handler");
    run(opt.port, rx).await
}
