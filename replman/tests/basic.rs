use difference::assert_diff;
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
    #[replman(alias = "exit")]
    #[replman(starts_with = "q")]
    Quit,
    /// Just here to mess with quit starts_with
    Quote,
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
    /// Lorem ipsum
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
        help - Displays help
        quit|exit|q.. - Exits the program

                        make sure to use it
        quote - Just here to mess with quit starts_with
        named_args <left> <right>
        unnamed_args <0> <1>
        optional_arg <first_arg> <optional_arg>
        with_default_value <with_default_value> - A command with a single argument that has a default (type based) value
                                                  Lorem ipsum
        with_default_explicit <with_default_explicit> - A command with a single argument that has a default value
        str <0>
    "#};

    assert_diff!(HELP, Command::help(), "", 0);
}

#[test_case("q")]
#[test_case("qu")]
#[test_case("qui")]
#[test_case("quit")]
#[test_case("exit")]
fn handles_aliases(s: &str) {
    let cmd = Command::parse_str(s).unwrap();

    assert_eq!(Command::Quit, cmd);
}

#[test]
fn starts_with_is_handled_last() {
    let cmd = Command::parse_str("quote").unwrap();

    assert_eq!(Command::Quote, cmd);
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
