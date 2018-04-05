use std::result::Result;
use std::process::exit;
use std::str::from_utf8;
use util::options::Options;
use url::Url;
use ws::{CloseCode, Error as WsError, Frame, Handler, Handshake, OpCode, Request, Response,
         Result as WsResult, Sender};

pub struct Client {
    pub out: Sender,
    pub options: Options,
}

impl Handler for Client {
    /// A method for creating the initial handshake request for WebSocket clients.
    /// Used to set headers on the initial request.
    fn build_request(&mut self, url: &Url) -> Result<Request, WsError> {
        let mut req = Request::from_url(url)?;
        req.headers_mut()
            .push(("Origin".into(), get_origin(url).into()));

        add_headers(req.headers_mut(), &self.options.headers);

        if self.options.print_headers {
            eprintln!("WebSocket upgrade request");
            eprintln!("---");
            eprintln!("{}", req);
        }

        Ok(req)
    }

    fn on_response(&mut self, res: &Response) -> WsResult<()> {
        if self.options.print_headers {
            eprintln!("WebSocket upgrade response");
            eprintln!("---");
            eprintln!("{}", res);
        }

        Ok(())
    }

    fn on_open(&mut self, _: Handshake) -> WsResult<()> {
        eprintln!("Connected to {}", self.options.url);
        Ok(())
    }

    fn on_frame(&mut self, frame: Frame) -> WsResult<Option<Frame>> {
        match frame.opcode() {
            OpCode::Text => {
                println!("{}", String::from_utf8(frame.payload().to_vec()).unwrap());
            }
            OpCode::Binary => {
                if !self.options.silent {
                    eprintln!("Recieved a binary frame, but this is not supported. See https://github.com/getwurl/wurl/issues/4");
                }
            }
            OpCode::Ping => {
                if self.options.show_control_frames {
                    println!(
                        "[ping] {}",
                        String::from_utf8(frame.payload().to_vec()).unwrap()
                    );
                }
            }
            OpCode::Pong => {
                if self.options.show_control_frames {
                    println!(
                        "[pong] {}",
                        String::from_utf8(frame.payload().to_vec()).unwrap()
                    );
                }
            }
            OpCode::Close => {
                if self.options.show_control_frames {
                    let close_code = &frame.payload()[..2];
                    let raw_code: u16 =
                        (u16::from(close_code[0]) << 8) | (u16::from(close_code[1]));
                    let named = CloseCode::from(raw_code);

                    trace!(
                        "Connection received raw close code: {:?}, {:?}",
                        raw_code,
                        close_code
                    );

                    if let Ok(reason) = from_utf8(&frame.payload()[2..]) {
                        println!(
                            "[close] {:?} ({}) {}",
                            named,
                            raw_code,
                            reason,
                        );
                    } else {
                        println!(
                            "[close] {:?} ({})",
                            named,
                            raw_code,
                        );
                    }
                }
            }
            _ => {}
        };

        Ok(Some(frame))
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

fn add_headers(request_headers: &mut Vec<(String, Vec<u8>)>, input_headers: &Vec<String>) {
    for header in input_headers {
        let split: Vec<&str> = header.split(':').collect();
        let key = split.first().unwrap().trim();
        let value = split.last().unwrap().trim();
        request_headers.push((key.to_owned().into(), value.to_owned().into()));
    }
}
