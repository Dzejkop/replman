use replman::prelude::*;

#[derive(PartialEq, Debug, ReplCmd)]
#[replman(rename_all = "snake_case")]
pub enum Command {
    /// Displays help
    #[replman(alias = "h")]
    Help,
    /// Exits the program
    ///
    /// make sure to use it
    #[replman(alias = "exit")]
    #[replman(starts_with = "q")]
    Quit,
    Quote,
    NamedArg {
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
    let mut repl = Repl::new();
    loop {
        let command: Command = repl.read_command()?;

        match command {
            Command::Help => println!("{}", Command::help()),
            Command::Quit => break,
            Command::Quote => {
                println!("Veni, Vidi, Vici");
            }
            cmd => println!("{:?}", cmd),
        }
    }

    Ok(())
}
