<h1 align="center">
  <a href="https://github.com/getwurl/wurl">
    <img src="assets/logo.png" alt="wurl" />
  </a>
</h1>

> The WebSocket CLI for developers

[![Build Status](https://travis-ci.org/getwurl/wurl.svg?branch=master)](https://travis-ci.org/getwurl/wurl)

WebSockets can be hard to work with due to their stateful nature. It is not
really feasible to just open a websocket in a web browser like you can with
a RESTful API, so tools are needed. wurl (pronounced _'whirl'_) is that tool.

It is a ground up rewrite of [wsta][wsta]. Still heavy work in progress, but
usable.

![Example of usage](assets/example.gif)

## Contents

- [Highlights](#highlights)
- [Usage](#usage)
- [Control Frames](#control-frames)
- [Examples](#examples)
- [Plugins](#plugins)
- [Wurl vs wsta](#wurl-vs-wsta)
- [Install](#install)

## Highlights

- Transmit messages to and from WebSocket servers from a CLI
- Composable by design, works great with UNIX pipes
- Easy to learn
- Inherently scriptable
- Follows the UNIX philosophy of doing one thing, and doing it well
- Written in rust, providing type safety and thread safety
- Extensible as a rust library

## Usage

```
USAGE:
    wurl [FLAGS] [OPTIONS] <URL>
FLAGS:
    -e, --echo       Enables echoing of outgoing frames
    -h, --help       Prints help information
    -i, --include    Include the HTTP headers in the output
    -s, --silent     Supresses all output except incoming frames
    -V, --version    Prints version information
    -v, --verbose    Increments verbosity by one
OPTIONS:
    -H, --header <header:value>...    Adds headers to the WebSocket HTTP handshake
    -C, --control-frames <type>       Enables echoing of control frames [possible values: all, in, out]
ARGS:
    <URL>    The URL of the server to connect to
```

### Connect to a server

To connect to a server with `wurl`, you pass it the only required parameter, a
URL of the WebSocket server.

    $ wurl wss://echo.websocket.org
    Connected to wss://echo.websocket.org

After this, `wurl` listens to stdin and sends any lines it reads to the
server as messages. WebSocket control frames are also supported, see the
[control frames section](#control-frames) for more information about that.

If you pass the `-e, --echo` flag, you will see outgoing frames as well. These
are always prefixed with `>` and a space to ensure they are possible to filter
out.

### Showing more information

You may sometimes encounter issues connecting to a server. In this case it may
be useful to see more information about the WebSocket upgrade request. In order
to do this, pass the `-i` flag.

```
$ wurl -i wss://echo.websocket.org
WebSocket upgrade request
---
GET / HTTP/1.1
Connection: Upgrade
Host: echo.websocket.org:443
Sec-WebSocket-Version: 13
Sec-WebSocket-Key: bIkIvw9EcchOo931ELJpOg==
Upgrade: websocket
Origin: https://echo.websocket.org


WebSocket upgrade response
---
HTTP/1.1 101 Web Socket Protocol Handshake
Connection: Upgrade
Date: Mon, 09 Apr 2018 18:00:10 GMT
Sec-WebSocket-Accept: 9j72nbYPuMMhmEpRl4xmN+YnZDI=
Server: Kaazing Gateway
Upgrade: websocket


Connected to wss://echo.websocket.org
```

Displaying control frames may be useful, for example to see if your server is
pinging correctly.

### Control frames

It will be supported to send control frames to the server using commands. The
currently available commands are:

- `/ping` - Sends a ping frame
- `/ping <message>` - Sends a ping frame with the given message
- `/pong` - Sends a pong frame
- `/pong <message>` - Sends a pong frame with the given message
- `/close <code>` - Sends a close code to the server
- `/close <code> <msg>` - Sends a close code with a given message to the server

If you need to send a frame which contains a leading slash like the above
commands, add a leading space to escape it.

### Authentication

Authentication is not built in to `wurl`, but setting custom headers is
possible. Use the `-H` option to set as many authentication headers as you need.

For an easy way of setting an authentication header automatically, see
[wurl-auth][wurl_auth].

## Examples


## Plugins

The main wurl executable is intentionally kept tiny. Many other programs
integrate well `wurl`, and some provide features you may need. Here are some
that are designed to work well with `wurl`.

* [wurl-auth][wurl_auth] - Set authentication headers
* [wurl-tools][wurl_tools] - Easily automate sending of messages

_Feel free to add to this list if you created something cool._

## wurl vs wsta
### Pros
- Ground up rewrite using a modern rust toolchain and libraries
- (Should be) Much faster and handle higher throughput due to the new async,
  event driven architecture and no locking
- Is now a library in addition to a CLI which makes it possible for rust
  programmers to programmatically control wurl
- Supports control frames
- Follows the UNIX philosophy, and is just as pipe-friendly as wsta
- Supports musl-based OSes (like alpine linux)
- Supports LibreSSL-based OSes??
- OSes via first-party distributed docker image

### Cons
- Does not support binary data. If this is requested enough, it may in the
  future. Please see [this issue][binary_issue]
- No man pages (yet)
- Does not have first party distribution through OS package managers. The system
  used by wsta was not satisfactory, and will not be used again. Packaging will
  have to be done by volunteers. (Pst, want to
  [be a packager?](#other-distribution-systems))

## Install
First party distribution:
- [crates.io](#cratesio)
- [binaries](#binaries)
- [docker](#docker)

Third party distibution:
- None yet

### [crates.io][crates.io]

If you are a rust programmer, it is easy to install wurl using the cargo CLI.

    $ cargo install wurl

Remember to make sure that `$HOME/.cargo/bin` is on your `$PATH`, or you will
not be able to find the installed binary.

To update to a newer version, simply overwrite the old executable when
installing.

    $ cargo install --force wurl

### [binaries][binaries]

All tagged releases on GitHub have compiled binaries attached to them. See the
[releases page][binaries] for the download links for your system.

### [docker][docker]

    $ docker run -it --rm --network=host esphen/wurl

If you do not have the rust toolchain installed and there is no package for your
OS, you may use the docker image, which will run on all platforms that docker
supports.

If you use docker, it may be an idea to add an alias to make running it through
docker easy. For example:

    $ alias wurl="docker run -it --rm --network=host esphen/wurl"

### Other distribution systems

Would you like to maintain a package for your OS? If it is not listed above,
feel free to add it to the package manager of your choice. Once you have done
so, please make a [pull request][pull_request] to add it to the list above.

[wsta]: https://github.com/esphen/wsta/
[crates.io]: https://crates.io
[binaries]: https://github.com/getwurl/wurl/releases
[docker]: https://store.docker.com/community/images/getwurl/wurl
[binary_issue]: https://github.com/getwurl/wurl/issues/4
[wurl_auth]: https://github.com/getwurl/wurl-auth
[wurl_tools]: https://github.com/getwurl/wurl-tools
[pull_request]: https://github.com/getwurl/wurl/issues/new?title=New%20package:%20%3CInsert%20OS%20or%20package%20here%3E&labels=packages
