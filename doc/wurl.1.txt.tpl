wurl(1)
=======

Name
----
wurl - communicate with WebSocket servers


Synopsis
--------
*wurl* [_OPTIONS_] [--] [_URL_]

*wurl* [_OPTIONS_] *--help*

*wurl* [_OPTIONS_] *--version*


DESCRIPTION
-----------
WebSocket curl (wurl) does things. Fill in things here!


POSITIONAL ARGUMENTS
--------------------
_URL_::
  The URL of the server to connect to


OPTIONS
-------
{OPTIONS}


EXIT STATUS
-----------
If the connection terminated with a close code of 1000, wurl exits with the exit
code 0 to signal a successful termination. Otherwise it will exit with the close
code of the WebSocket close control frame as the exit code.

An error internally in wurl will result in an exit code of 1.


SHELL COMPLETION
----------------
Shell completion files are included in the release tarball for Bash, Fish, Zsh
and PowerShell.

For *bash*, move `wurl.bash` to `$XDG_CONFIG_HOME/bash_completion`
or `/etc/bash_completion.d/`.

For *fish*, move `wurl.fish` to `$HOME/.config/fish/completions`.

For *zsh*, move `_wurl` to one of your `$fpath` directories.


CAVEATS
-------
TODO


VERSION
-------
{VERSION}


HOMEPAGE
--------
https://github.com/getwurl/wurl

Please report bugs and feature requests in the issue tracker.


AUTHORS
-------
Espen Henriksen <dev+wurlman@henriksen.is>

