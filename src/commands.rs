use crate::resp::Resp;

#[derive(Debug)]
pub enum Command {
    Get(Vec<u8>),
    Set((Vec<u8>, Vec<u8>)),
    Command,
    Config(String),
}

impl From<Resp> for Command {
    fn from(object: Resp) -> Self {
        match object {
            Resp::Array(Some(elements)) => parse_redis_command(elements),
            _ => panic!("Command can only be created from Array"),
        }
    }
}

fn parse_redis_command(elements: Vec<Resp>) -> Command {
    let command = match &elements[0] {
        Resp::BulkString(Some(val)) => String::from_utf8_lossy(val),
        _ => panic!("First element of a command must be bulk string."),
    };

    match command.to_lowercase().as_str() {
        "get" => parse_get(&elements[1..]),
        "set" => parse_set(&elements[1..]),
        "command" => parse_command(&elements[1..]),
        "config" => parse_config(&elements[1..]),
        _ => panic!("Unknown command: {}", command),
    }
}

fn parse_command(_arguments: &[Resp]) -> Command {
    Command::Command
}

fn parse_config(arguments: &[Resp]) -> Command {
    // if arguments.len() != 2 {
    //     panic!(
    //         "CONFIG command requires two arguments, {} were given",
    //         arguments.len()
    //     );
    // }
    let val = match &arguments[1] {
        Resp::BulkString(Some(val)) => val,
        _ => panic!("omg"),
    };

    Command::Config(String::from_utf8_lossy(val).to_string())
}

fn parse_set(arguments: &[Resp]) -> Command {
    if arguments.len() != 2 {
        panic!(
            "SET command requires two arguments, {} were given",
            arguments.len()
        );
    }
    let key = match &arguments[0] {
        Resp::BulkString(Some(val)) => val,
        _ => panic!("omg"),
    };
    let value = match &arguments[1] {
        Resp::BulkString(Some(val)) => val,
        _ => panic!("omg"),
    };
    Command::Set((key.clone(), value.clone()))
}

fn parse_get(arguments: &[Resp]) -> Command {
    if arguments.len() != 1 {
        panic!(
            "SET command requires two arguments, {} were given",
            arguments.len()
        );
    }
    let key = match &arguments[0] {
        Resp::BulkString(Some(val)) => val,
        _ => panic!("omg"),
    };
    Command::Get(key.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_get_command() {
        let resp = Resp::Array(Some(vec![
            Resp::BulkString(Some(b"GET".to_vec())),
            Resp::BulkString(Some(b"key".to_vec())),
        ]));
        let command = Command::from(resp);

        if let Command::Get(key) = command {
            assert_eq!(key, b"key".to_vec());
        } else {
            panic!("Error parsing GET command.");
        }
    }

    #[test]
    fn parses_lowercase_get_command() {
        let resp = Resp::Array(Some(vec![
            Resp::BulkString(Some(b"get".to_vec())),
            Resp::BulkString(Some(b"key".to_vec())),
        ]));
        let command = Command::from(resp);

        if let Command::Get(key) = command {
            assert_eq!(key, b"key".to_vec());
        } else {
            panic!("Error parsing GET command.");
        }
    }

    #[test]
    #[should_panic]
    fn panics_get_parsing_with_wrong_num_args() {
        let resp = Resp::Array(Some(vec![
            Resp::BulkString(Some(b"get".to_vec())),
            Resp::BulkString(Some(b"key".to_vec())),
            Resp::BulkString(Some(b"value".to_vec())),
        ]));
        Command::from(resp);
    }

    #[test]
    fn parses_set_command() {
        let resp = Resp::Array(Some(vec![
            Resp::BulkString(Some(b"SET".to_vec())),
            Resp::BulkString(Some(b"key".to_vec())),
            Resp::BulkString(Some(b"value".to_vec())),
        ]));
        let command = Command::from(resp);

        if let Command::Set((key, value)) = command {
            assert_eq!(key, b"key".to_vec());
            assert_eq!(value, b"value".to_vec());
        } else {
            panic!("Error parsing SET command.");
        }
    }

    #[test]
    fn parses_lowercase_set_command() {
        let resp = Resp::Array(Some(vec![
            Resp::BulkString(Some(b"set".to_vec())),
            Resp::BulkString(Some(b"key".to_vec())),
            Resp::BulkString(Some(b"value".to_vec())),
        ]));
        let command = Command::from(resp);

        if let Command::Set((key, value)) = command {
            assert_eq!(key, b"key".to_vec());
            assert_eq!(value, b"value".to_vec());
        } else {
            panic!("Error parsing SET command.");
        }
    }

    #[test]
    #[should_panic]
    fn panics_set_parsing_with_wrong_num_args() {
        let resp = Resp::Array(Some(vec![
            Resp::BulkString(Some(b"set".to_vec())),
            Resp::BulkString(Some(b"key".to_vec())),
        ]));
        Command::from(resp);
    }
}
