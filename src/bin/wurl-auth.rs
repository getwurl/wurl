#[macro_use]
extern crate clap;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use hyper::{Client, Uri};
use hyper::client::Response;
use hyper::header::SetCookie;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use clap::App;

fn main() {
    let yaml = load_yaml!("wurl-auth.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let url = value_t_or_exit!(matches, "url", Uri);

    match fetch(url) {
        Ok(mut response) => {
            let cookies = response.headers_mut().get::<SetCookie>().unwrap();

            let mut cookie_values = Vec::new();
            for cookie in cookies.iter() {
                // Get only key=value part of cookie
                let split = cookie.split(';').collect::<Vec<&str>>();
                if let Some(header) = split.first() {
                    cookie_values.push(header.clone());
                }
            }

            print!("Cookie: {}", cookie_values.join("; "));
        },
        Err(error) => eprintln!("An error occured while fetching: {}", error),
    }
}

fn fetch(uri: Uri) -> Result<Response, hyper::Error> {
    let mut core = Core::new()?;
    let client = Client::configure()
        .connector(HttpsConnector::new(1, &core.handle()).unwrap())
        .build(&core.handle());

    core.run(client.get(uri))
}
