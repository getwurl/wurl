//! The command line options provided to the program
use std::vec::Vec;
use std::str::FromStr;

/// An error that occurs when parsing of an option occurs
#[derive(Clone, Debug)]
pub struct OptionParseError {
    /// The human-readable error message
    reason: String,
}

/// An enumeration for identifying the direction frames are headed.
#[derive(Clone, Debug, PartialEq)]
pub enum Show {
    /// Does not match any headers
    None,
    /// Matches both incoming and outgoing frames
    All,
    /// Matches only incoming (ingress) frames
    Incoming,
    /// Matches only outgoing (egress) frames
    Outgoing,
}

impl FromStr for Show {
    type Err = OptionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "all" => Ok(Show::All),
            "in" => Ok(Show::Incoming),
            "out" => Ok(Show::Outgoing),
            _ => Err(OptionParseError {
                reason: format!("{} is not valid", s),
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Options {
    /// Supresses all output except incoming frames
    pub silent: bool,

    /// The WebSocket URL to connect to.
    pub url: String,

    /// Specifies when to print control frames. Control frames will be prefixed
    /// with the type of control frame, for example "[ping]", and further
    /// prefixed with ">" for outgoing control frames
    pub show_control_frames: Show,

    /// Print outgoing frames as well as the incoming frames. Outgoing
    /// frames will be prefixed with ">".
    pub echo: bool,

    /// Print the headers of any HTTP request when true.
    pub print_headers: bool,

    /// Headers
    pub headers: Vec<String>,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            url: String::new(),
            show_control_frames: Show::None,
            echo: false,
            silent: false,
            print_headers: false,
            headers: Vec::new(),
        }
    }
}

impl Options {
    /// Construct options for iterating over *none* of the symbol kinds.
    pub fn nothing() -> Options {
        Options {
            url: String::new(),
            show_control_frames: Show::None,
            echo: false,
            silent: false,
            print_headers: false,
            headers: Vec::new(),
        }
    }
}
