pub mod commands;
pub mod error;
pub mod resp;
pub mod server;
pub mod store;

use std::future::Future;
use std::sync::{Arc, Mutex};

use error::SerirError;
use tokio::net::TcpListener;

use server::Server;
use store::KeyValueStore;
use tokio::select;

pub async fn run(port: u16, sigint: impl Future) -> Result<(), SerirError> {
    let store = Arc::new(Mutex::new(KeyValueStore::new()));
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    let server = Server::new(store, listener);

    select! {
        _ = server.run() => {

        },
        _ = sigint => {

        }
    }
    Ok(())
}
