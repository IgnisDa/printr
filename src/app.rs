use clap::{self, crate_authors, crate_version, App, AppSettings};

/// Arg is a light alias for a clap::Arg that is specialized to compile time
/// string literals.
type Arg = clap::Arg<'static, 'static>;

pub struct CustomArg {
    /// The underlying clap argument.
    clap_arg: Arg,
    /// The name of this argument. This is always present and is the name
    /// used in the code to find the value of an argument at runtime.
    pub name: &'static str,
    /// A short documentation string describing this argument. This string
    /// should fit on a single line and be a complete sentence.
    ///
    /// This is shown in the `-h` output.
    pub doc_short: &'static str,
    /// A longer documentation string describing this argument. This usually
    /// starts with the contents of `doc_short`. This is also usually many
    /// lines, potentially paragraphs, and may contain examples and additional
    /// prose.
    ///
    /// This is shown in the `--help` output.
    /// Whether this flag is hidden or not.
    ///
    /// Generally, these flags should be documented in the documentation for
    /// the flag they override.
    pub hidden: bool,
    pub doc_long: &'static str,
    /// The type of this argument.
    pub kind: CustomArgKind,
}
#[derive(Clone)]
pub enum CustomArgKind {
    /// A positional argument.
    Positional {
        /// The name of the value used in the `-h/--help` output. By
        /// convention, this is an all-uppercase string. e.g., `PATH` or
        /// `PATTERN`.
        value_name: &'static str,
        /// Whether an argument can be repeated multiple times or not.
        multiple: bool,
    },
    /// A boolean switch.
    Switch {
        /// The long name of a flag. This is always non-empty.
        long: &'static str,
        /// The short name of a flag. This is empty if a flag only has a long
        /// name.
        short: Option<&'static str>,
        /// Whether this switch can be provided multiple times where meaning
        /// is attached to the number of times this flag is given.
        ///
        /// Note that every switch can be provided multiple times. This
        /// particular state indicates whether all instances of a switch are
        /// relevant or not.
        multiple: bool,
    },
    /// A flag the accepts a single value.
    Flag {
        /// The long name of a flag. This is always non-empty.
        long: &'static str,
        /// The short name of a flag. This is empty if a flag only has a long
        /// name.
        short: Option<&'static str>,
        /// The name of the value used in the `-h/--help` output. By
        /// convention, this is an all-uppercase string. e.g., `PATH` or
        /// `PATTERN`.
        value_name: &'static str,
        /// Whether this flag can be provided multiple times with multiple
        /// distinct values.
        ///
        /// Note that every flag can be provided multiple times. This
        /// particular state indicates whether all instances of a flag are
        /// relevant or not.
        multiple: bool,
        /// A set of possible values for this flag. If an end user provides
        /// any value other than what's in this set, then clap will report an
        /// error.
        possible_values: Vec<&'static str>,
    },
}

impl CustomArg {
    /// Create a positional argument.
    ///
    /// The `long_name` parameter is the name of the argument, e.g., `pattern`.
    /// The `value_name` parameter is a name that describes the type of
    /// argument this flag accepts. It should be in uppercase, e.g., PATH or
    /// PATTERN.
    fn positional(name: &'static str, value_name: &'static str) -> Self {
        Self {
            clap_arg: Arg::with_name(name).value_name(value_name),
            name,
            doc_short: "",
            doc_long: "",
            hidden: false,
            kind: CustomArgKind::Positional {
                value_name,
                multiple: false,
            },
        }
    }

    /// Create a boolean switch.
    ///
    /// The `long_name` parameter is the name of the flag, e.g., `--long-name`.
    ///
    /// All switches may be repeated an arbitrary number of times. If a switch
    /// is truly boolean, that consumers of clap's configuration should only
    /// check whether the flag is present or not. Otherwise, consumers may
    /// inspect the number of times the switch is used.
    fn switch(long_name: &'static str) -> Self {
        let clap_arg = Arg::with_name(long_name).long(long_name);
        Self {
            clap_arg,
            name: long_name,
            doc_short: "",
            doc_long: "",
            hidden: false,
            kind: CustomArgKind::Switch {
                long: long_name,
                short: None,
                multiple: false,
            },
        }
    }

    /// Create a flag. A flag always accepts exactly one argument.
    ///
    /// The `long_name` parameter is the name of the flag, e.g., `--long-name`.
    /// The `value_name` parameter is a name that describes the type of
    /// argument this flag accepts. It should be in uppercase, e.g., PATH or
    /// PATTERN.
    ///
    /// All flags may be repeated an arbitrary number of times. If a flag has
    /// only one logical value, that consumers of clap's configuration should
    /// only use the last value.
    fn flag(long_name: &'static str, value_name: &'static str) -> Self {
        let clap_arg = Arg::with_name(long_name)
            .long(long_name)
            .value_name(value_name)
            .takes_value(true)
            .number_of_values(1);
        Self {
            clap_arg,
            name: long_name,
            doc_short: "",
            doc_long: "",
            hidden: false,
            kind: CustomArgKind::Flag {
                long: long_name,
                short: None,
                value_name,
                multiple: false,
                possible_values: vec![],
            },
        }
    }

    /// Set the short flag name.
    ///
    /// This panics if this arg isn't a switch or a flag.
    fn short(mut self, name: &'static str) -> Self {
        match self.kind {
            CustomArgKind::Positional { .. } => panic!("expected switch or flag"),
            CustomArgKind::Switch { ref mut short, .. } => {
                *short = Some(name);
            }
            CustomArgKind::Flag { ref mut short, .. } => {
                *short = Some(name);
            }
        }
        self.clap_arg = self.clap_arg.short(name);
        self
    }

    /// Set the "short" help text.
    ///
    /// This should be a single line. It is shown in the `-h` output.
    fn help(mut self, text: &'static str) -> Self {
        self.doc_short = text;
        self.clap_arg = self.clap_arg.help(text);
        self
    }

    /// Set the "long" help text.
    ///
    /// This should be at least a single line, usually longer. It is shown in
    /// the `--help` output.
    fn long_help(mut self, text: &'static str) -> Self {
        self.doc_long = text;
        self.clap_arg = self.clap_arg.long_help(text);
        self
    }

    /// Enable this argument to accept multiple values.
    ///
    /// Note that while switches and flags can always be repeated an arbitrary
    /// number of times, this particular method enables the flag to be
    /// logically repeated where each occurrence of the flag may have
    /// significance. That is, when this is disabled, then a switch is either
    /// present or not and a flag has exactly one value (the last one given).
    /// When this is enabled, then a switch has a count corresponding to the
    /// number of times it is used and a flag's value is a list of all values
    /// given.
    ///
    /// For the most part, this distinction is resolved by consumers of clap's
    /// configuration.
    fn multiple(mut self) -> Self {
        match self.kind {
            CustomArgKind::Positional {
                ref mut multiple, ..
            } => {
                *multiple = true;
            }
            CustomArgKind::Switch {
                ref mut multiple, ..
            } => {
                *multiple = true;
            }
            CustomArgKind::Flag {
                ref mut multiple, ..
            } => {
                *multiple = true;
            }
        }
        self.clap_arg = self.clap_arg.multiple(true);
        self
    }

    /// Set the possible values for this argument. If this argument is not
    /// a flag, then this panics.
    ///
    /// If the end user provides any value other than what is given here, then
    /// clap will report an error to the user.
    ///
    /// Note that this will suppress clap's automatic output of possible values
    /// when using -h/--help, so users of this method should provide
    /// appropriate documentation for the choices in the "long" help text.
    fn possible_values(mut self, values: &[&'static str]) -> Self {
        match self.kind {
            CustomArgKind::Positional { .. } => panic!("expected flag"),
            CustomArgKind::Switch { .. } => panic!("expected flag"),
            CustomArgKind::Flag {
                ref mut possible_values,
                ..
            } => {
                *possible_values = values.to_vec();
                self.clap_arg = self
                    .clap_arg
                    .possible_values(values)
                    .hide_possible_values(true);
            }
        }
        self
    }

    /// Sets this argument to a required argument, unless one of the given
    /// arguments is provided.
    #[allow(dead_code)]
    fn required_unless(mut self, names: &[&'static str]) -> Self {
        self.clap_arg = self.clap_arg.required_unless_one(names);
        self
    }

    /// Hide this flag from all documentation.
    #[allow(dead_code)]
    fn hidden(mut self) -> Self {
        self.hidden = true;
        self.clap_arg = self.clap_arg.hidden(true);
        self
    }

    /// Add an alias to this argument.
    ///
    /// Aliases are not show in the output of -h/--help.
    #[allow(dead_code)]
    fn alias(mut self, name: &'static str) -> Self {
        self.clap_arg = self.clap_arg.alias(name);
        self
    }

    /// Permit this flag to have values that begin with a hyphen.
    ///
    /// This panics if this arg is not a flag.
    #[allow(dead_code)]
    fn allow_leading_hyphen(mut self) -> Self {
        match self.kind {
            CustomArgKind::Positional { .. } => panic!("expected flag"),
            CustomArgKind::Switch { .. } => panic!("expected flag"),
            CustomArgKind::Flag { .. } => {
                self.clap_arg = self.clap_arg.allow_hyphen_values(true);
            }
        }
        self
    }

    /// Sets conflicting arguments. That is, if this argument is used whenever
    /// any of the other arguments given here are used, then clap will report
    /// an error.
    fn conflicts(mut self, names: &[&'static str]) -> Self {
        self.clap_arg = self.clap_arg.conflicts_with_all(names);
        self
    }

    /// Sets an overriding argument. That is, if this argument and the given
    /// argument are both provided by an end user, then the "last" one will
    /// win.
    #[allow(dead_code)]
    fn overrides(mut self, name: &'static str) -> Self {
        self.clap_arg = self.clap_arg.overrides_with(name);
        self
    }

    /// Sets the default value of this argument when not specified at
    /// runtime.
    #[allow(dead_code)]
    fn default_value(mut self, value: &'static str) -> Self {
        self.clap_arg = self.clap_arg.default_value(value);
        self
    }

    /// Sets the default value of this argument if and only if the argument
    /// given is present.
    #[allow(dead_code)]
    fn default_value_if(mut self, value: &'static str, arg_name: &'static str) -> Self {
        self.clap_arg = self.clap_arg.default_value_if(arg_name, None, value);
        self
    }

    /// Indicate that any value given to this argument should be a number. If
    /// it's not a number, then clap will report an error to the end user.
    #[allow(dead_code)]
    fn number(mut self) -> Self {
        self.clap_arg = self.clap_arg.validator(|val| {
            val.parse::<usize>()
                .map(|_| ())
                .map_err(|err| err.to_string())
        });
        self
    }
}

// We add an extra space to long descriptions so that a blank line is inserted
// between flag descriptions in --help output.
macro_rules! long {
    ($lit:expr) => {
        concat!($lit, " ")
    };
}

const ABOUT: &str = "
printr is meant to be a drop in replacement for the everyday shell command - echo. Though
coloring is supported by echo, it is un-intuitive and often not cross-platform. Enter printr,
a cross platform command that does all that echo does, and a bit more.

It supports coloring as well as styling your output via simple command line flags. Most of
the time, however, supplying these flags is not required since printr is smart enough to
figure out the input STRING's sentiment via some naive sentiment analysis.

Project home page: https://github.com/IgnisDa/printr
";

pub fn app() -> App<'static, 'static> {
    let mut app = App::new("printr")
        .author(crate_authors!())
        .version(crate_version!())
        .about(ABOUT)
        .max_term_width(100)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::AllArgsOverrideSelf)
        .help_message("Prints help information. Use --help for more details.");
    for arg in all_args_and_flags() {
        app = app.arg(arg.clap_arg);
    }
    app
}

pub fn all_args_and_flags() -> Vec<CustomArg> {
    let mut args = vec![];
    arg_string(&mut args);
    switch_newline(&mut args);
    switch_spaces(&mut args);
    switch_enable_interpretation(&mut args);
    switch_disable_interpretation(&mut args);
    switch_plain(&mut args);
    switch_error(&mut args);
    flag_input_file(&mut args);
    flag_color(&mut args);
    flag_formatting(&mut args);
    args
}

fn arg_string(args: &mut Vec<CustomArg>) {
    const SHORT: &str = "The string to print";
    const LONG: &str = long!(
        "\
A string that will be printed to stdout (or stderr using --error flag).
This can be left empty, in which case, a newline will be printed
(assuming the -n/--newline flag is not supplied).

Example:

    `printr Hello` will output \"Hello\"
    `printr Hello World` will output \"Hello World\"
        "
    );
    let arg = CustomArg::positional("STRING", "STRING")
        .multiple()
        .help(SHORT)
        .long_help(LONG)
        .conflicts(&["input-file"]);
    args.push(arg);
}

fn switch_newline(args: &mut Vec<CustomArg>) {
    const SHORT: &str = "Do not output a newline at the end";
    const LONG: &str = long!(
        "\
When this switch is specified, the output does not have a newline
(that is, a '\\n') appended to its end.
        "
    );
    let arg = CustomArg::switch("newline")
        .short("n")
        .help(SHORT)
        .long_help(LONG);
    args.push(arg);
}

fn switch_spaces(args: &mut Vec<CustomArg>) {
    const SHORT: &str = "Do not separate arguments with spaces";
    const LONG: &str = long!(
        "\
When this switch is specified, input STRING arguments are not
separated by spaces.

Example:

    `printr -s Hello World` will output \"HelloWorld\"

This switch has no effect on the output if an input file
(via the --input-file option) is supplied.
        "
    );
    let arg = CustomArg::switch("spaces")
        .short("s")
        .help(SHORT)
        .long_help(LONG);
    args.push(arg);
}

fn switch_disable_interpretation(args: &mut Vec<CustomArg>) {
    const SHORT: &str = "Disable interpretation of backslash escapes (default)";
    const LONG: &str = long!(
        "\
This is the default behavior.
        "
    );
    let arg = CustomArg::switch("disable_interpretation")
        .short("E")
        .help(SHORT)
        .long_help(LONG);
    args.push(arg);
}

fn switch_enable_interpretation(args: &mut Vec<CustomArg>) {
    const SHORT: &str = "Enable interpretation of backslash escapes";
    const LONG: &str = long!(
        "\
If -e is used, the following sequences are recognized:

    • \\ backslash

    • \\a alert (BEL)

    • \\b backspace

    • \\c produce no further output

    • \\e escape

    • \\f form feed

    • \\n new line

    • \\r carriage return

    • \\t horizontal tab

    • \\v vertical tab

    • \\0NNN byte with octal value NNN (1 to 3 digits)

    • \\xHH byte with hexadecimal value HH (1 to 2 digits)
        "
    );
    let arg = CustomArg::switch("enable_interpretation")
        .short("e")
        .help(SHORT)
        .long_help(LONG);
    args.push(arg);
}

fn switch_plain(args: &mut Vec<CustomArg>) {
    const SHORT: &str = "Print the input without any color";
    const LONG: &str = long!(
        "\
This option prints the output \"plainly\", that is, no colors are applied.
It is helpful when piping the output of printr to another file, so that
color escape codes do not mangle up the resulting file.
        "
    );
    let arg = CustomArg::switch("plain")
        .short("p")
        .help(SHORT)
        .long_help(LONG);
    args.push(arg);
}

fn switch_error(args: &mut Vec<CustomArg>) {
    const SHORT: &str = "Print to stderr instead of stdout";
    const LONG: &str = long!(
        "\
If this switch is supplied, the output will be printed to stdout.
This can then be piped as required.

Example:

        printr --error \"Error string input\" 2> errors.txt
        "
    );
    let arg = CustomArg::switch("error").help(SHORT).long_help(LONG);
    args.push(arg);
}

fn flag_input_file(args: &mut Vec<CustomArg>) {
    const SHORT: &str = "The file to read the input STRING from";
    const LONG: &str = long!(
        "\
If this is supplied, the input STRING is read from the file
contents.
        "
    );
    let arg = CustomArg::flag("input-file", "input-file")
        .short("i")
        .help(SHORT)
        .long_help(LONG)
        .conflicts(&["STRING"]);
    args.push(arg);
}

fn flag_color(args: &mut Vec<CustomArg>) {
    const SHORT: &str = "The color of the output";
    const LONG: &str = long!(
        "\
If not supplied, the color is guessed from the contents of the input
using some naive sentiment analysis.
[possible values: red, blue, green, yellow, cyan]
        "
    );
    let arg = CustomArg::flag("color", "color")
        .short("c")
        .help(SHORT)
        .long_help(LONG)
        .conflicts(&["plain"])
        .possible_values(&["red", "blue", "green", "yellow", "cyan"]);
    args.push(arg);
}

fn flag_formatting(args: &mut Vec<CustomArg>) {
    const SHORT: &str = "The style to apply to the text, defaults to `None`";
    const LONG: &str = long!(
        "\
Some terminal emulators do not support some of the formatting options.
You might have to experiment to find which ones work for you.
[possible values: bold, underline, strikethrough, dimmed]
        "
    );
    let arg = CustomArg::flag("formatting", "formatting")
        .short("f")
        .help(SHORT)
        .long_help(LONG)
        .possible_values(&["bold", "underline", "strikethrough", "dimmed"]);
    args.push(arg);
}
