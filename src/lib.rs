pub mod commands;
pub mod resp;
pub mod server;
pub mod store;

use std::error::Error;

use std::sync::{Arc, Mutex};

use tokio::net::TcpListener;

use server::Server;
use store::KeyValueStore;
use tokio::select;
use tokio::sync::oneshot::Receiver;

pub async fn run(port: u16, running: Receiver<bool>) -> Result<(), Box<dyn Error>> {
    let store = Arc::new(Mutex::new(KeyValueStore::new()));
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    let server = Server::new(store, listener);

    select! {
        _ = server.run() => {

        },
        _ = running => {

        }
    }
    Ok(())
}
