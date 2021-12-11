use std::io::{stdin, stdout, Write};

pub mod prelude {
    pub use crate::read_command;
    pub use crate::ReplCmd;
    pub use replman_derive::ReplCmd;
}

pub trait ReplCmd {
    fn help() -> &'static str;
    fn parse(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub fn read_command<R>() -> anyhow::Result<R>
where
    R: ReplCmd,
{
    let mut line = String::new();
    loop {
        line.clear();
        print!("> ");
        stdout().flush()?;
        stdin().read_line(&mut line)?;

        if line.trim().is_empty() {
            continue;
        }

        match R::parse(line.trim()) {
            Ok(cmd) => return Ok(cmd),
            Err(err) => eprintln!("Failed to parse command: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use replman_derive::ReplCmd;

    #[derive(PartialEq, Debug, ReplCmd)]
    #[replman(rename_all = "snake_case")]
    enum Command {
        /// Displays help
        Help,
        /// Exits the program
        ///
        /// make sure to use it
        Quit,
        Add {
            left: usize,
            right: usize,
        },
        Mul(usize, usize),
        Echo {
            name: String,
        },
        SendEchoRequest {
            job_address: String,
            value: u32,
        },
    }

    #[test]
    fn help_test() {
        const HELP: &str = indoc::indoc! {r#"
            h|help - displays help
            quit - Exits the program

                   make sure to use it
            add <left> <right>
            mul <0> <1>
            echo <name>
            send_echo_request <job_address> <value>
        "#};

        assert_eq!(HELP, Command::help());
    }

    #[test]
    fn add() {
        let cmd = Command::parse("add 1 2").unwrap();

        assert_eq!(Command::Add { left: 1, right: 2 }, cmd);
    }

    #[test]
    fn mul() {
        let cmd = Command::parse("mul 1 2").unwrap();

        assert_eq!(Command::Mul(1, 2), cmd);
    }

    #[test]
    fn send_echo_request() {
        let cmd = Command::parse("send_echo_request Hello 2").unwrap();

        assert_eq!(
            Command::SendEchoRequest {
                job_address: "Hello".to_string(),
                value: 2,
            },
            cmd
        );
    }
}
