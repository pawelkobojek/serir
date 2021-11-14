use std::collections::HashMap;

use crate::commands::Command;
use crate::resp::Resp;

#[derive(Debug)]
pub struct KeyValueStore {
    store: HashMap<Vec<u8>, Vec<u8>>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn exec(&mut self, command: Command) -> Vec<u8> {
        match command {
            Command::Get(key) => self.get(&key),
            Command::Set((key, value)) => self.set(&key, value),
            Command::Command => Resp::BulkString(None).serialize(),
            Command::Config(value) if value == *"save" => b"*2\r\n$4\r\nsave\r\n$23\r\n3600 1 300 100 60 10000\r\n".to_vec(),
            Command::Config(value) if value == *"appendonly" => b"*2\r\n$10\r\nappendonly\r\n$2\r\nno\r\n".to_vec(),
            Command::Config(_) => Resp::Error(b"Supporting only \"appendonly\" and \"save\"".to_vec()).serialize(),
        }
    }

    fn store_set(&mut self, key: &[u8], value: Vec<u8>) {
        self.store.insert(key.to_owned(), value);
    }

    fn store_get(&self, key: &[u8]) -> Option<&Vec<u8>> {
        self.store.get(&key.to_owned())
    }

    fn get(&self, key: &[u8]) -> Vec<u8> {
        let value = match self.store_get(key) {
            Some(val) => Resp::BulkString(Some(val.clone())),
            None => Resp::BulkString(None),
        };

        value.serialize()
    }

    fn set(&mut self, key: &[u8], value: Vec<u8>) -> Vec<u8> {
        self.store_set(key, value);

        Resp::SimpleString(b"OK".to_vec()).serialize()
    }
}

impl Default for KeyValueStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn sets_and_gets_values() {
        let mut store = KeyValueStore::new();

        let key_len = thread_rng().gen_range(2..=100);
        let key: Vec<u8> = (0..key_len).map(|_| thread_rng().gen::<u8>()).collect();

        let value_len = thread_rng().gen_range(2..=100);
        let value: Vec<u8> = (0..value_len).map(|_| thread_rng().gen::<u8>()).collect();

        store.store_set(&key, value.clone());

        if let Some(get_value) = store.store_get(&key) {
            assert_eq!(value, *get_value);
        } else {
            panic!("No value in store");
        }
    }
}
