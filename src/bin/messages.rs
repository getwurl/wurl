use std::io::{Error, ErrorKind};
use std::collections::vec_deque::VecDeque;

#[derive(PartialEq, Debug)]
pub enum Kind {
    Message,
    Ping,
    Pong,
    Close,
}

#[derive(PartialEq, Debug)]
pub struct Message {
    pub kind: Kind,
    pub message: Option<String>,
    pub code: Option<u16>,
}

impl Message {
    fn from_message(message: String) -> Message {
        Message {
            kind: Kind::Message,
            code: None,
            message: Some(message),
        }
    }

    fn from_ping(input: String) -> Message {
        let suffix = input.replace("/ping", "");
        let message = suffix.trim();
        Message {
            kind: Kind::Ping,
            code: None,
            message: if !message.is_empty() {
                Some(message.to_owned())
            } else {
                None
            },
        }
    }

    fn from_pong(input: String) -> Message {
        let suffix = input.replace("/pong", "");
        let message = suffix.trim();
        Message {
            kind: Kind::Pong,
            code: None,
            message: if !message.is_empty() {
                Some(message.to_owned())
            } else {
                None
            },
        }
    }

    fn from_close(input: String) -> Message {
        let mut split = input.split_whitespace().collect::<VecDeque<&str>>();
        split.pop_front();
        let code = split.pop_front();
        let message = join_deque(&mut split);
        Message {
            kind: Kind::Close,
            code: if let Some(item) = code {
                Some(item.parse::<u16>().unwrap())
            } else {
                None
            },
            message: if !message.is_empty() {
                Some(message.to_owned())
            } else {
                None
            },
        }
    }
}

pub fn parse_message(message: String) -> Result<Message, Error> {
    if message.starts_with('/') {
        if message.starts_with("/ping") {
            return Ok(Message::from_ping(message));
        }

        if message.starts_with("/pong") {
            return Ok(Message::from_pong(message));
        }

        if message.starts_with("/close") {
            return Ok(Message::from_close(message));
        }

        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                "Unrecognized command. If you meant to send this as a message, add a leading space to escape the command"
            )
        );
    }

    Ok(Message::from_message(message.trim().to_string()))
}

fn join_deque(deque: &mut VecDeque<&str>) -> String {
    let first = deque.pop_front();
    if first.is_none() {
        return String::new();
    }

    let mut result = String::from(first.unwrap());
    for item in deque.iter() {
        result = format!("{} {}", result, item);
    }

    String::from(result)
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;
    use messages::{parse_message, Kind, Message};

    #[test]
    fn test_parse_message() {
        let mut actual = parse_message(String::from("Hello there"));
        assert!(actual.is_ok(), "Normal message did not return successfully");
        assert_eq!(
            Message {
                kind: Kind::Message,
                message: Some(String::from("Hello there")),
                code: None,
            },
            actual.unwrap(),
            "Normal message was not parsed correctly"
        );

        actual = parse_message(String::from("/ping"));
        assert!(actual.is_ok(), "Ping command did not return successfully");
        assert_eq!(
            Message {
                kind: Kind::Ping,
                message: None,
                code: None,
            },
            actual.unwrap(),
            "Ping command was not parsed correctly"
        );

        actual = parse_message(String::from("/ping pinging you"));
        assert!(
            actual.is_ok(),
            "Ping command with message did not return successfully"
        );
        assert_eq!(
            Message {
                kind: Kind::Ping,
                message: Some(String::from("pinging you")),
                code: None,
            },
            actual.unwrap(),
            "Ping command with message was not parsed correctly"
        );

        actual = parse_message(String::from("/pong"));
        assert!(actual.is_ok(), "Pong command did not return successfully");
        assert_eq!(
            Message {
                kind: Kind::Pong,
                message: None,
                code: None,
            },
            actual.unwrap(),
            "Pong command was not parsed correctly"
        );

        actual = parse_message(String::from("/pong ponging you"));
        assert!(
            actual.is_ok(),
            "Pong command with message did not return successfully"
        );
        assert_eq!(
            Message {
                kind: Kind::Pong,
                message: Some(String::from("ponging you")),
                code: None,
            },
            actual.unwrap(),
            "Pong command with message was not parsed correctly"
        );

        actual = parse_message(String::from("/close"));
        assert!(actual.is_ok(), "Close command did not return successfully");
        assert_eq!(
            Message {
                kind: Kind::Close,
                message: None,
                code: None,
            },
            actual.unwrap(),
            "Close command was not parsed correctly"
        );

        actual = parse_message(String::from("/close 1000"));
        assert!(
            actual.is_ok(),
            "Close command with code did not return successfully"
        );
        assert_eq!(
            Message {
                kind: Kind::Close,
                message: None,
                code: Some(1000),
            },
            actual.unwrap(),
            "Close command with code was not parsed correctly"
        );

        actual = parse_message(String::from("/close 1000 close down, you"));
        assert!(
            actual.is_ok(),
            "Close command with code and message did not return successfully"
        );
        assert_eq!(
            Message {
                kind: Kind::Close,
                message: Some(String::from("close down, you")),
                code: Some(1000),
            },
            actual.unwrap(),
            "Close command with code and message was not parsed correctly"
        );

        actual = parse_message(String::from("/foobar"));
        assert!(actual.is_err(), "Invalid command did not return error");
        assert_eq!(
            ErrorKind::InvalidInput,
            actual.unwrap_err().kind(),
            "Close command with code and message was not parsed correctly"
        );

        actual = parse_message(String::from(" /foobar"));
        assert!(
            actual.is_ok(),
            "Escaped invalid command did not return successfully"
        );
        assert_eq!(
            Message {
                kind: Kind::Message,
                message: Some(String::from("/foobar")),
                code: None,
            },
            actual.unwrap(),
            "Escaped invalid command was not parsed correctly"
        );
    }
}
