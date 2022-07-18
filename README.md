# Replman
[![API](https://docs.rs/replman/badge.svg)](https://docs.rs/replman)
[![example workflow](https://github.com/Dzejkop/replman/actions/workflows/rust.yml/badge.svg)](https://github.com/Dzejkop/replman/actions/workflows/rust.yml)

An opinionated REPL framework.

Have you ever wanted to quickly create a REPL program, but got annoyed at having to parse commands yourself?

Did you wish there was something like `StructOpt` but for REPL commands?

If you answered yes to any of the above questions - then this crate is for you!

`Replman` makes writing REPL tools a breeze.

## Example
```rust
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
    /// Returns left * right
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
```
