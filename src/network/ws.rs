use std::result::Result;
use std::process::exit;
use std::thread;
use util::options::Options;
use url::Url;
use ws::{WebSocket, Handler, Sender, Handshake, Result as WsResult, Error as WsError, ErrorKind, Message, CloseCode};

// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
struct Client {
    out: Sender,
    options: Options,
}

// We implement the Handler trait for Client so that we can get more
// fine-grained control of the connection.
impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> WsResult<()> {
      eprintln!("Connected to {}", self.options.url);
      self.out.send("Hello WebSocket")
    }

    fn on_message(&mut self, msg: Message) -> WsResult<()> {
      // Close the connection when we get a response from the server
      println!("{}", msg);
      self.out.close(CloseCode::Normal)
    }

    fn on_error(&mut self, err: WsError) {
      eprintln!("Error occured in WebSocket thread: {:?}", err);
    }

    fn on_shutdown(&mut self) {
      eprintln!("Request to shutdown recieved from server. Exiting.");
      exit(1)
    }
}

pub fn connect(options: Options) -> Result<Sender, WsError> {
    let url = options.url.clone();
    let mut ws = try!(WebSocket::new(move |out| Client { out: out, options: options.clone() }));
    let parsed = try!(
        Url::parse(&url)
            .map_err(|err| WsError::new(
                ErrorKind::Internal,
                format!("Unable to parse {} as url due to {:?}", &url, err))));
    try!(ws.connect(parsed));
    let sender = ws.broadcaster();

    thread::Builder::new().name("websocket_handler".to_owned()).spawn(move || {
        // This blocks the thread
        ws.run()
    });

    Ok(sender)
}