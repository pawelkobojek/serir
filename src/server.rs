use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::commands::Command;
use crate::error::SerirResult;
use crate::resp::Resp;
use crate::store::KeyValueStore;

pub struct Server {
    store: Arc<Mutex<KeyValueStore>>,
    listener: TcpListener,
}

async fn handle_client(
    store: Arc<Mutex<KeyValueStore>>,
    mut socket: TcpStream,
) -> SerirResult<()> {
    let mut buffer = vec![0; 1024];
    loop {
        let bytes_read = socket.read(&mut buffer).await?;
        if bytes_read == 0 {
            return Ok(());
        }
        let inputs = Resp::deserialize(&buffer[..bytes_read])?;
        let mut response = vec![];
        for input in inputs {
            let command = Command::from(input);
            let mut result = store.lock().unwrap().exec(command)?;
            response.append(&mut result);
        }
        socket.write_all(&response).await?;
        buffer = vec![0; 1024];
    }
}

impl Server {
    pub fn new(store: Arc<Mutex<KeyValueStore>>, listener: TcpListener) -> Server {
        Server { store, listener }
    }

    pub async fn run(&self) -> SerirResult<()> {
        loop {
            let (socket, _) = self.listener.accept().await?;
            let store = self.store.clone();
            tokio::spawn(async move {
                match handle_client(store, socket).await {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error reading from stream: {}", e);
                    }
                }
            });
        }
    }
}
