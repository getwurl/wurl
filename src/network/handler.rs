use std::result::Result;
use std::process::exit;
use util::options::Options;
use url::Url;
use ws::{CloseCode, Error as WsError, Handler, Handshake, Message, Request, Result as WsResult,
         Sender};

// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
pub struct Client {
    pub out: Sender,
    pub options: Options,
}

// We implement the Handler trait for Client so that we can get more
// fine-grained control of the connection.
impl Handler for Client {
    /// A method for creating the initial handshake request for WebSocket clients.
    /// Used to set headers on the initial request.
    fn build_request(&mut self, url: &Url) -> Result<Request, WsError> {
        let mut req = Request::from_url(url)?;
        req.headers_mut()
            .push(("Origin".into(), get_origin(url).into()));
        Ok(req)
    }

    fn on_open(&mut self, _: Handshake) -> WsResult<()> {
        eprintln!("Connected to {}", self.options.url);
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> WsResult<()> {
        println!("{}", msg);
        Ok(())
    }

    fn on_error(&mut self, err: WsError) {
        eprintln!("Error occured: {}", err);
        eprintln!("Exiting");
        self.out
            .close(CloseCode::Normal)
            .expect("Failed to close socket");
        exit(1);
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        if reason.is_empty() {
            eprintln!("Recieved close control frame. Exit code: {:?}", code);
        } else {
            eprintln!(
                "Recieved close control frame. Reason: {} ({:?})",
                reason, code
            );
        }
        eprintln!("Exiting");
        exit(1);
    }

    fn on_shutdown(&mut self) {
        eprintln!("Request to shutdown recieved from server. Exiting.");
        exit(1)
    }
}

/// Parses an Origin string from a websocket URL, replacing ws[s] with http[s].
fn get_origin(url: &Url) -> String {
    let scheme = if url.scheme() == "wss" {
        "https"
    } else {
        "http"
    };

    format!("{}://{}", scheme, url.host_str().unwrap_or(""))
}
