use replman::prelude::*;
use test_case::test_case;

#[derive(PartialEq, Debug, ReplCmd)]
#[replman(rename_all = "snake_case")]
enum Command {
    /// Displays help
    Help,
    /// Exits the program
    ///
    /// make sure to use it
    Quit,
    NamedArgs {
        left: usize,
        right: usize,
    },
    UnnamedArgs(usize, usize),
    OptionalArg {
        first_arg: String,
        optional_arg: Option<u32>,
    },
    /// A command with a single argument that has a default (type based) value
    WithDefaultValue {
        #[replman(default)]
        with_default_value: u32,
    },
    /// A command with a single argument that has a default value
    WithDefaultExplicit {
        #[replman(default = "42")]
        with_default_explicit: u32,
    },
    Str(String),
}

#[test]
fn help_test() {
    const HELP: &str = indoc::indoc! {r#"
        h|help - displays help
        quit - Exits the program

               make sure to use it
        named_args <left> <right>
        unnamed_args <0> <1>
        echo <name>
        send_echo_request <job_address> <value>
    "#};

    assert_eq!(HELP, Command::help());
}

#[test]
fn named_args() {
    let cmd = Command::parse_str("named_args 1 2").unwrap();

    assert_eq!(Command::NamedArgs { left: 1, right: 2 }, cmd);
}

#[test]
fn unnamed_args() {
    let cmd = Command::parse_str("unnamed_args 1 2").unwrap();

    assert_eq!(Command::UnnamedArgs(1, 2), cmd);
}

#[test_case("optional_arg Hello", Command::OptionalArg {
    first_arg: "Hello".to_string(), optional_arg: None
} ; "optional not set")]
#[test_case("optional_arg World 1", Command::OptionalArg {
    first_arg: "World".to_string(), optional_arg: Some(1)
} ; "optional set")]
fn optional_arg(s: &str, exp: Command) {
    let cmd = Command::parse_str(s).unwrap();
    assert_eq!(exp, cmd);
}

#[test_case("with_default_value", Command::WithDefaultValue {
    with_default_value: 0,
} ; "default")]
#[test_case("with_default_value 24", Command::WithDefaultValue {
    with_default_value: 24,
} ; "default override")]
fn with_default_value(s: &str, exp: Command) {
    let cmd = Command::parse_str(s).unwrap();
    assert_eq!(exp, cmd);
}

#[test_case("with_default_explicit", Command::WithDefaultExplicit {
    with_default_explicit: 42,
}; "default explicit")]
#[test_case("with_default_explicit 24", Command::WithDefaultExplicit {
    with_default_explicit: 24,
}; "default explicit override")]
fn with_default_explicit(s: &str, exp: Command) {
    let cmd = Command::parse_str(s).unwrap();
    assert_eq!(exp, cmd);
}

#[test_case("str \"Hello, World!\"", Command::Str("Hello, World!".to_string()) ; "double quotes")]
#[test_case("str 'Hello, World!'", Command::Str("Hello, World!".to_string()) ; "single quotes")]
#[test_case("str \"'Hello', World!\"", Command::Str("'Hello', World!".to_string()) ; "single quotes within double")]
#[test_case("str \"\"", Command::Str("".to_string()) ; "empty str")]
fn escape_strings(s: &str, exp: Command) {
    let cmd = Command::parse_str(s).unwrap();
    assert_eq!(exp, cmd);
}
