# wsta 2

Ground up rewrite of [wsta][wsta]. Heavy work in progress. Working name is wsy,
but is subject to change (please suggest more names!).

[![Build Status](https://travis-ci.org/esphen/wsta2.svg?branch=master)](https://travis-ci.org/esphen/wsta2)


## Distribution
First party distribution:
- [crates.io](#crates.io)
- [docker](#docker)

Third party distibution:
- None yet

### [crates.io][crates.io]

    $ cargo install wsy

If you are a rust programmer, it is easy to install wsy using the cargo CLI.

### [docker][docker]

    $ docker run -it --rm --network=host esphen/wsy

If you do not have the rust toolchain installed and there is no package for your
OS, you may use the docker image, which will run on all platforms that docker
supports.

If you use docker, it may be an idea to add an alias to make running it through
docker easy. For example:

    $ alias wsy="docker run -it --rm --network=host esphen/wsy"

### Other distribution systems

Would you like to maintain a package for your OS? If it is not listed above,
feel free to add it to the package manager of your choice. Once you have done
so, please make a [pull request][pull_request] to add it to the list above.

[wsta]: https://github.com/esphen/wsta/
[crates.io]: https://crates.io
[docker]:
[pull_request]: https://github.com/esphen/wsta2/issues/new?title=New%20package:%20%3CInsert%20OS%20or%20package%20here%3E&labels=packages
