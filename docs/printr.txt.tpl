print(1)
========

Name
----
printr - the smarter echo alternative

Synopsis
--------
printr [FLAGS] [OPTIONS] [STRING]...

Flags
-----
    -E               Disable interpretation of backslash escapes (default)
    -e               Enable interpretation of backslash escapes
        --error      Print to stderr instead of stdout
    -h, --help       Prints help information
    -n, --newline    Do not output a newline at the end
    -p, --plain      Print the input without any color
    -s, --spaces     Do not separate arguments with spaces, does nothing if --file is supplied
    -V, --version    Prints version information

Options
-------
    -c <color>                       If not supplied, the output color will be guessed from the
                                     context [possible values: red, blue, green, yellow, cyan]
    -f, --formatting <formatting>    The style to apply to the text, defaults to `None` [possible
                                     values: bold, underline, strikethrough, dimmed]
    -i, --input-file <input-file>    The file to read the input STRING from

VERSION
-------
{{VERSION}}

HOMEPAGE
--------
https://github.com/IgnisDa/printr

Please report bugs and feature requests in the issue tracker. Please do your
best to provide a reproducible test case for bugs.

AUTHORS
-------
{{AUTHORS}}
