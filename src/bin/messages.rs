use ws::Sender;
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

pub fn parse_message(message: String) -> Message {
    if message.starts_with('/') {
        if message.starts_with("/ping") {
            return Message::from_ping(message);
        }

        if message.starts_with("/pong") {
            return Message::from_pong(message);
        }

        if message.starts_with("/close") {
            return Message::from_close(message);
        }
    }

    Message::from_message(message)
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

fn send_message(message: String, sender: &Sender) {
    if message.trim().is_empty() {
        return;
    }
}

#[cfg(test)]
mod tests {
    use messages::{parse_message, Kind, Message};

    #[test]
    fn test_parse_message() {
        assert_eq!(
            parse_message(String::from("Hello there")),
            Message {
                kind: Kind::Message,
                message: Some(String::from("Hello there")),
                code: None,
            }
        );

        assert_eq!(
            parse_message(String::from("/ping")),
            Message {
                kind: Kind::Ping,
                message: None,
                code: None,
            }
        );

        assert_eq!(
            parse_message(String::from("/ping pinging you")),
            Message {
                kind: Kind::Ping,
                message: Some(String::from("pinging you")),
                code: None,
            }
        );

        assert_eq!(
            parse_message(String::from("/pong")),
            Message {
                kind: Kind::Pong,
                message: None,
                code: None,
            }
        );

        assert_eq!(
            parse_message(String::from("/pong ponging you")),
            Message {
                kind: Kind::Pong,
                message: Some(String::from("ponging you")),
                code: None,
            }
        );

        assert_eq!(
            parse_message(String::from("/close")),
            Message {
                kind: Kind::Close,
                message: None,
                code: None,
            }
        );

        assert_eq!(
            parse_message(String::from("/close 1000")),
            Message {
                kind: Kind::Close,
                message: None,
                code: Some(1000),
            }
        );

        assert_eq!(
            parse_message(String::from("/close 1000 close down, you")),
            Message {
                kind: Kind::Close,
                message: Some(String::from("close down, you")),
                code: Some(1000),
            }
        );
    }
}
