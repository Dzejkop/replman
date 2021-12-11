use replman::prelude::*;

#[derive(Debug, Clone, ReplCmd)]
#[replman(rename_all = "snake_case")]
enum Cmd {
    Quit,
    Help,
    Add(i64, i64),
    Addf(f64, f64),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    loop {
        let cmd: Cmd = read_command()?;

        match cmd {
            Cmd::Quit => break,
            Cmd::Help => println!("{}", Cmd::help()),
            Cmd::Add(a, b) => println!("{}", a + b),
            Cmd::Addf(a, b) => println!("{}", a + b),
        }
    }

    Ok(())
}
