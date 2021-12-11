use replman::prelude::*;

#[derive(PartialEq, Debug, ReplCmd)]
#[replman(rename_all = "snake_case")]
pub enum Command {
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
    WithDefaultValue {
        #[replman(default)]
        with_default_value: u32,
    },
    WithDefaultExplicit {
        #[replman(default = "42")]
        with_default_explicit: u32,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Ok(())
}
