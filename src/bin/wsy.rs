#[macro_use]
extern crate clap;
extern crate rprompt;
extern crate wsy;

use std::process::exit;
use rprompt::read_reply;
use wsy::util::options::Options;
use wsy::network::ws::connect;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let mut app = App::from_yaml(yaml)
        .name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!());
    let matches = app.clone().get_matches();
    let mut opts = Options::default();

    if let Some(url) = matches.value_of("URL") {
        opts.url = String::from(url);
    }

    opts.verbosity = matches.occurrences_of("verbose") as u8;

    if opts.url.is_empty() {
        app.print_help().expect("Failed to print help message");
        exit(1);
    }

    let sender = match connect(opts) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Failed to connect to WebSocket server: {}", error);
            exit(1);
        }
    };

    loop {
        match read_reply() {
            Ok(input) => {
                if !input.trim().is_empty() {
                    sender
                        .send(input)
                        .expect("Failed to send WebSocket message");
                }
            }
            Err(error) => eprintln!("error: {}", error),
        }
    }
}
