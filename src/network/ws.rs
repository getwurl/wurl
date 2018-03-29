use std::result::Result;
use std::thread;
use util::options::Options;
use url::Url;
use network::handler::Client;
use ws::{Error as WsError, ErrorKind, Sender, WebSocket};

pub fn connect(options: Options) -> Result<Sender, WsError> {
    let url = options.url.clone();
    let mut ws = WebSocket::new(move |out| Client {
        out: out,
        options: options.clone(),
    })?;
    let parsed = Url::parse(&url).map_err(|err| {
        WsError::new(
            ErrorKind::Internal,
            format!("Unable to parse {} as url due to {:?}", &url, err),
        )
    })?;
    ws.connect(parsed)?;
    let sender = ws.broadcaster();

    thread::Builder::new()
        .name("websocket_handler".to_owned())
        .spawn(move || {
            // This blocks the thread
            ws.run()
        })
        .expect("Failed to start WebSocket thread");

    Ok(sender)
}
