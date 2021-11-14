use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub enum Resp {
    SimpleString(Vec<u8>),
    Integer(i64),
    Error(Vec<u8>),
    BulkString(Option<Vec<u8>>),
    Array(Vec<Resp>), // nil array not supported (yet?)
}

impl Resp {
    pub fn deserialize(buffer: &[u8]) -> Vec<Self> {
        Parser::new(buffer).parse()
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];

        match self {
            Resp::BulkString(Some(val)) => {
                buffer.write_all(b"$").unwrap();
                buffer
                    .write_all(&format!("{}\r\n", val.len()).into_bytes())
                    .unwrap();
                buffer.write_all(val).unwrap();
                buffer.write_all(b"\r\n").unwrap();
            }
            Resp::BulkString(None) => buffer.write_all(b"$-1\r\n").unwrap(),
            Resp::Integer(val) => {
                buffer.write_all(b":").unwrap();
                buffer
                    .write_all(&format!("{}\r\n", val).into_bytes())
                    .unwrap();
            }
            Resp::SimpleString(val) => {
                buffer.write_all(b"+").unwrap();
                buffer.write_all(val).unwrap();
                buffer.write_all(b"\r\n").unwrap();
            }
            Resp::Error(val) => {
                buffer.write_all(b"-").unwrap();
                buffer.write_all(val).unwrap();
                buffer.write_all(b"\r\n").unwrap();
            }
            Resp::Array(val) => {
                buffer.write_all(b"*").unwrap();
                buffer
                    .write_all(&format!("{}\r\n", val.len()).into_bytes())
                    .unwrap();
                for item in val {
                    buffer.write_all(&item.serialize()).unwrap();
                }
            }
        }

        buffer
    }
}

pub struct Parser<'a> {
    reader: BufReader<&'a [u8]>,
}

impl<'a> Parser<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            reader: BufReader::new(buffer),
        }
    }

    pub fn parse_single_resp_object(&mut self) -> Option<Resp> {
        let mut buf = [0u8; 1];
        if self.reader.read_exact(&mut buf).is_err() {
            return None;
        };
        let type_byte = buf[0];
        let resp = match type_byte {
            b'*' => self.parse_array(),
            b'$' => self.parse_bulk_string(),
            b':' => self.parse_integer(),
            b'+' => self.parse_simple_string(),
            b'-' => self.parse_error(),
            _ => panic!("Unsupported byte type: {}", type_byte as char),
        };
        Some(resp)
    }

    pub fn parse(&mut self) -> Vec<Resp> {
        let mut resps = vec![];
        loop {
            let resp = self.parse_single_resp_object();
            match resp {
                Some(resp) => resps.push(resp),
                None => break,
            };
        }
        resps
    }

    fn parse_len(&mut self) -> isize {
        let mut line = String::new();
        self.reader.read_line(&mut line).unwrap();
        line.trim_end().parse::<isize>().unwrap()
    }

    fn parse_array(&mut self) -> Resp {
        let len = self.parse_len();
        let values: Vec<Resp> = (0..len).map(|_| {
            let parsed = self.parse_single_resp_object();
            parsed.unwrap()
        }).collect();
        Resp::Array(values)
    }

    fn parse_bulk_string(&mut self) -> Resp {
        let len = self.parse_len();
        if len < 0 {
            return Resp::BulkString(None);
        }

        let len = len as usize;
        let mut buf = vec![0u8; (len + 2) as usize];
        self.reader.read_exact(&mut buf).unwrap();
        if buf.split_off(len) != b"\r\n" {
            todo!();
        }
        Resp::BulkString(Some(buf))
    }

    fn parse_integer(&mut self) -> Resp {
        let mut line = String::new();
        self.reader.read_line(&mut line).unwrap();

        Resp::Integer(line.trim_end().parse::<i64>().unwrap())
    }

    fn parse_simple_string(&mut self) -> Resp {
        let mut line = String::new();
        self.reader.read_line(&mut line).unwrap();
        Resp::SimpleString(line.trim_end().as_bytes().to_vec())
    }

    fn parse_error(&mut self) -> Resp {
        let mut line = String::new();
        self.reader.read_line(&mut line).unwrap();
        Resp::Error(line.trim_end().as_bytes().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bulk_string() {
        let result = Resp::deserialize(&b"$5\r\nSerir\r\n"[..]);
        assert_eq!(result.len(), 1);
        if let Resp::BulkString(Some(result)) = &result[0] {
            assert_eq!(*result, b"Serir".to_vec());
        } else {
            panic!("Parsing error");
        }
    }

    #[test]
    fn parses_nil_bulk_string() {
        let result = &Resp::deserialize(&b"$-1\r\n"[..])[0];
        assert!(matches!(*result, Resp::BulkString(None)));
    }

    #[test]
    fn parses_arrays() {
        // TODO: Somethings seriously wrong with this test - must be non-idiomatic
        let parsed = Resp::deserialize(&b"*2\r\n$2\r\nOK\r\n$5\r\nSerir\r\n"[..]);
        assert_eq!(parsed.len(), 1);
        let result = &parsed[0];
        if let Resp::Array(result) = result {
            if let Resp::BulkString(Some(s)) = &result[0] {
                assert_eq!(*s, b"OK".to_vec());
            } else {
                panic!("Parsing error: not a bulk string");
            }

            if let Resp::BulkString(Some(s)) = &result[1] {
                assert_eq!(*s, b"Serir".to_vec());
            } else {
                panic!("Parsing error: not a bulk string");
            }
        } else {
            panic!("Parsing error");
        }
    }

    #[test]
    fn parses_empty_arrays() {
        let result = &Resp::deserialize(&b"*0\r\n"[..])[0];
        if let Resp::Array(val) = result {
            assert_eq!(val.len(), 0);
        } else {
            panic!("Error parsing empty array");
        }
    }

    #[test]
    fn parses_integers() {
        let result = &Resp::deserialize(&b":42\r\n"[..])[0];
        if let Resp::Integer(result) = result {
            assert_eq!(*result, 42);
        } else {
            panic!("Error parsing integer");
        }
    }

    #[test]
    fn parses_simple_strings() {
        let result = &Resp::deserialize(&b"+OK - seems good.\r\n"[..])[0];
        if let Resp::SimpleString(result) = result {
            assert_eq!(*result, b"OK - seems good.".to_vec());
        } else {
            panic!("Error parsing integer");
        }
    }

    #[test]
    fn parses_errors() {
        let result = &Resp::deserialize(&b"-Error message\r\n"[..])[0];
        if let Resp::Error(result) = result {
            assert_eq!(*result, b"Error message".to_vec());
        } else {
            panic!("Error parsing integer");
        }
    }

    #[test]
    fn parses_multiple_objects_sent_at_once() {
        let result = Resp::deserialize(&b"$1\r\nA\r\n$1\r\nB\r\n"[..]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn parses_multiple_objects_with_arrays_sent_at_once() {
        let result = Resp::deserialize(&b"*1\r\n$1\r\nA\r\n*1\r\n$1\r\nB\r\n"[..]);
        assert_eq!(result.len(), 2);
    }
}
