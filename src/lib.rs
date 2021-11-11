pub mod commands;
pub mod resp;
pub mod store;

use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::{
    error::Error,
    net::{TcpListener, TcpStream},
};

use commands::Command;
use store::KeyValueStore;

use rayon::ThreadPoolBuilder;

use crate::resp::Resp;

fn handle_client(
    store: Arc<Mutex<KeyValueStore>>,
    mut conn: TcpStream,
) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![0; 1024];
    while conn.read(&mut buffer)? > 0 {
        let input = Resp::deserialize(&buffer[..]);
        let command = Command::from(input);
        let result = store.lock().unwrap().exec(command);

        conn.write_all(&result)?;
        buffer = vec![0; 1024];
    }
    Ok(())
}

pub fn run(port: u16, num_workers: usize) -> Result<(), Box<dyn Error>> {
    let pool = ThreadPoolBuilder::new().num_threads(num_workers).build()?;
    let store = Arc::new(Mutex::new(KeyValueStore::new()));
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;

    for conn in listener.incoming() {
        let conn = conn?;
        let store = store.clone();
        pool.spawn(move || match handle_client(store, conn) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
            }
        });
    }

    Ok(())
}
