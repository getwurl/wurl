# wurl - WebSocket curl

> The WebSocket CLI for developers

wurl (pronounced _'whirl'_) is a ground up rewrite of [wsta][wsta]. Heavy work
in progress. The working name is wurl, but is subject to change (please suggest
more names in [#3][suggest_names]!).

[![Build Status](https://travis-ci.org/esphen/wurl.svg?branch=master)](https://travis-ci.org/esphen/wurl)

## wurl vs wsta
### Pros
- Ground up rewrite using a modern rust toolchain and libraries
- (Should be) Much faster and handle higher throughput due to the new async,
  event driven architecture and no locking
- Is now a library in addition to a CLI which makes it possible for rust
  programmers to programmatically control wurl
- Supports control frames
- Follows the unix philosophy, and is just as pipe-friendly as wsta
- Supports musl-based OSes via first-party distributed docker image

### Cons
- Does not support binary data. If this is requested enough, it may in the
  future. Please see [this issue][binary_issue]
- Does not have first party distribution through OS package managers. This will
  have to be done by volunteers. (Pst, want to
  [be a packager?](#other-distribution-systems))

## Control frames

It will be supported to send control frames to the server using commands. These
will be:

- `/ping` - Sends a ping frame
- `/ping <message>` - Sends a ping frame with the given message
- `/pong` - Sends a pong frame
- `/pong <message>` - Sends a pong frame with the given message
- `/close <code>` - Sends a close code to the server
- `/close <code> <msg>` - Sends a close code with a given message to the server

If you need to send a frame which contains a leading slash like the above
commands, add a leading space to escape it. For example ` /foobar`.


## Distribution
First party distribution:
- [crates.io](#cratesio)
- [docker](#docker)

Third party distibution:
- None yet

### [crates.io][crates.io]

    $ cargo install wurl

If you are a rust programmer, it is easy to install wurl using the cargo CLI.

To update to a newer version, simply overwrite the old executable when
installing.

    $ cargo install --force wurl

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
[docker]: https://store.docker.com/community/images/esphen/wurl
[binary_issue]: https://github.com/esphen/wurl/issues/4
[suggest_names]: https://github.com/esphen/wurl/issues/3
[pull_request]: https://github.com/esphen/wurl/issues/new?title=New%20package:%20%3CInsert%20OS%20or%20package%20here%3E&labels=packages
