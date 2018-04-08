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

- [Usage](#usage)
- [Control Frames](#control-frames)
- [Plugins](#plugins)
- [Wurl vs wsta](#wurl-vs-wsta)
- [Install](#install)

## Usage

TODO

## Control frames

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
- Follows the unix philosophy, and is just as pipe-friendly as wsta
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

> NOT YET PUBLISHED

    $ cargo install wurl

If you are a rust programmer, it is easy to install wurl using the cargo CLI.

To update to a newer version, simply overwrite the old executable when
installing.

    $ cargo install --force wurl

### [binaries][binaries]

All tagged releases on GitHub have compiled binaries attached to them.

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
