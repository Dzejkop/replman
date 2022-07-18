use replman::prelude::*;

#[derive(PartialEq, Debug, Replman)]
#[replman(rename_all = "snake_case")]
pub enum Command {
    /// Displays help
    #[replman(alias = "h")]
    Help,
    /// Exits the program
    #[replman(alias = "exit")]
    #[replman(starts_with = "q")]
    Quit,
    /// Returns left + right
    Add {
        left: i64,
        right: i64,
    },
    Mul {
        left: i64,
        right: i64,
    },
}

fn main() -> anyhow::Result<()> {
    let mut repl = Repl::new();
    loop {
        let command: Command = repl.read_command()?;

        match command {
            Command::Help => println!("{}", Command::help()),
            Command::Quit => break,
            Command::Add { left, right } => {
                println!("{} + {} = {}", left, right, left + right)
            }
            Command::Mul { left, right } => {
                println!("{} * {} = {}", left, right, left * right)
            }
        }
    }

    Ok(())
}
