use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{
    mpsc::{self, Sender},
    oneshot,
};

use crate::commands::Command;
use crate::error::SerirResult;
use crate::resp::Resp;
use crate::store::KeyValueStore;

pub struct Server {
    store: Arc<Mutex<KeyValueStore>>,
    listener: TcpListener,
}

#[derive(Debug)]
pub struct Request {
    command: Command,
    response_tx: oneshot::Sender<Vec<u8>>,
}

async fn handle_client(commands_tx: Sender<Request>, mut socket: TcpStream) -> SerirResult<()> {
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
            let (response_tx, response_rx) = oneshot::channel();
            let request = Request {
                command,
                response_tx,
            };
            commands_tx.send(request).await?;
            let mut result = response_rx.await?;
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
        // TODO: make the channel buffer size configurable
        let (commands_tx, mut commands_rx) = mpsc::channel(10000);

        let store = self.store.clone();
        tokio::spawn(async move {
            while let Some(request) = commands_rx.recv().await {
                let Request {
                    command,
                    response_tx,
                } = request;
                let result = store.lock().unwrap().exec(command).unwrap();
                response_tx.send(result).unwrap();
            }
        });

        loop {
            let (socket, _) = self.listener.accept().await?;
            let commands_tx = commands_tx.clone();
            tokio::spawn(async move {
                match handle_client(commands_tx, socket).await {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error reading from stream: {}", e);
                    }
                }
            });
        }
    }
}
