use std::str::FromStr;

use rustyline::Editor;

pub mod prelude {
    pub use replman_derive::ReplCmd;

    pub use crate::{read_command, Repl, ReplCmd};
}

pub struct Repl {
    editor: Editor<()>,
}

impl Repl {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            editor: Editor::new(),
        }
    }

    pub fn read_command<R>(&mut self) -> anyhow::Result<R>
    where
        R: ReplCmd,
    {
        loop {
            let line = self.editor.readline("> ")?;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            match R::parse(split_string_unescape(trimmed)) {
                Ok(cmd) => {
                    self.editor.add_history_entry(trimmed);
                    return Ok(cmd);
                }
                Err(err) => eprintln!("Failed to parse command: {}", err),
            }
        }
    }
}

pub trait ReplCmd {
    fn help() -> &'static str;
    fn parse<'a, I>(parts: I) -> anyhow::Result<Self>
    where
        Self: Sized,
        I: Iterator<Item = anyhow::Result<&'a str>> + 'a;

    fn parse_str(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Self::parse(split_string_unescape(s))
    }
}

pub trait ReplCmdParse {
    fn parse(item: Option<&str>) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn parse_default(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized;
}

macro_rules! impl_with_from_str {
    ($t:ty) => {
        impl ReplCmdParse for $t {
            fn parse(item: Option<&str>) -> anyhow::Result<Self>
            where
                Self: Sized,
            {
                Ok(item
                    .ok_or_else(|| anyhow::anyhow!("Missing field"))?
                    .parse()?)
            }

            fn parse_default(s: &str) -> anyhow::Result<Self>
            where
                Self: Sized,
            {
                Ok(s.parse()?)
            }
        }
    };
}

impl_with_from_str!(std::net::IpAddr);
impl_with_from_str!(std::net::SocketAddr);
impl_with_from_str!(bool);
impl_with_from_str!(char);
impl_with_from_str!(f32);
impl_with_from_str!(f64);
impl_with_from_str!(i8);
impl_with_from_str!(i16);
impl_with_from_str!(i32);
impl_with_from_str!(i64);
impl_with_from_str!(i128);
impl_with_from_str!(isize);
impl_with_from_str!(u8);
impl_with_from_str!(u16);
impl_with_from_str!(u32);
impl_with_from_str!(u64);
impl_with_from_str!(u128);
impl_with_from_str!(usize);
impl_with_from_str!(std::ffi::OsString);
impl_with_from_str!(std::net::Ipv4Addr);
impl_with_from_str!(std::net::Ipv6Addr);
impl_with_from_str!(std::net::SocketAddrV4);
impl_with_from_str!(std::net::SocketAddrV6);
impl_with_from_str!(std::num::NonZeroI8);
impl_with_from_str!(std::num::NonZeroI16);
impl_with_from_str!(std::num::NonZeroI32);
impl_with_from_str!(std::num::NonZeroI64);
impl_with_from_str!(std::num::NonZeroI128);
impl_with_from_str!(std::num::NonZeroIsize);
impl_with_from_str!(std::num::NonZeroU8);
impl_with_from_str!(std::num::NonZeroU16);
impl_with_from_str!(std::num::NonZeroU32);
impl_with_from_str!(std::num::NonZeroU64);
impl_with_from_str!(std::num::NonZeroU128);
impl_with_from_str!(std::num::NonZeroUsize);
impl_with_from_str!(std::path::PathBuf);
impl_with_from_str!(String);

impl<T> ReplCmdParse for Option<T>
where
    T: FromStr,
    anyhow::Error: From<<T as FromStr>::Err>,
{
    fn parse(item: Option<&str>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(item.map(|s| s.parse()).transpose()?)
    }

    fn parse_default(_s: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        unimplemented!("Using default on an Option field makes no sense")
    }
}

pub fn read_command<R>() -> anyhow::Result<R>
where
    R: ReplCmd,
{
    let mut rl = Editor::<()>::new();

    loop {
        let line = rl.readline("> ")?;

        if line.trim().is_empty() {
            continue;
        }

        match R::parse(split_string_unescape(line.trim())) {
            Ok(cmd) => return Ok(cmd),
            Err(err) => eprintln!("Failed to parse command: {}", err),
        }
    }
}

fn split_string_unescape(
    mut s: &str,
) -> impl Iterator<Item = anyhow::Result<&str>> {
    std::iter::from_fn(move || {
        if s.is_empty() {
            return None;
        }

        let next_unescaped_space = find_next_unescaped_space(s);

        let ret = match next_unescaped_space {
            Ok(x) => match x {
                Some(x) => {
                    let ret = &s[..x];
                    s = &s[x + 1..];

                    ret
                }
                None => {
                    let temp = s;
                    s = "";

                    temp
                }
            },
            Err(err) => return Some(Err(err)),
        };

        Some(Ok(unescape(ret)))
    })
}

fn unescape(s: &str) -> &str {
    if (s.starts_with('"') && s.ends_with('"'))
        || (s.starts_with('\'') && s.ends_with('\''))
    {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

fn find_next_unescaped_space(s: &str) -> anyhow::Result<Option<usize>> {
    let mut is_in_double_qoutes = false;
    let mut is_in_single_quotes = false;

    let mut previous_was_quoted = false;
    for (idx, c) in s.char_indices() {
        if previous_was_quoted && c != ' ' {
            return Err(anyhow::anyhow!(
                "Invalid command fragment, expected a space or end of string, found '{}'",
                c
            ));
        }

        match c {
            '"' if !is_in_double_qoutes && !is_in_single_quotes => {
                is_in_double_qoutes = true
            }
            '"' if is_in_double_qoutes => {
                is_in_double_qoutes = false;
                previous_was_quoted = true;
            }
            '\'' if !is_in_double_qoutes && !is_in_single_quotes => {
                is_in_single_quotes = true
            }
            '\'' if is_in_single_quotes => {
                is_in_single_quotes = false;
                previous_was_quoted = true;
            }
            _ => previous_was_quoted = false,
        }

        if is_in_double_qoutes || is_in_single_quotes {
            continue;
        }

        if c == ' ' {
            return Ok(Some(idx));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("Hello", vec!["Hello"] ; "Single item")]
    #[test_case("Hello World!", vec!["Hello", "World!"] ; "Two items")]
    #[test_case("", vec![] ; "Empty")]
    fn basic(s: &str, exp: Vec<&str>) {
        let actual: Vec<_> =
            split_string_unescape(s).map(Result::unwrap).collect();
        assert_eq!(actual, exp);
    }

    #[test_case(r#""Hello, World!""#, vec!["Hello, World!"] ; "Single - double quotes")]
    #[test_case(r#"'Hello, World!'"#, vec!["Hello, World!"] ; "Single - single quotes")]
    #[test_case(r#"'Hello, World!' "What is going on?""#, vec!["Hello, World!", "What is going on?"] ; "Two items - mixed")]
    #[test_case(r#""" "" """#, vec!["", "", ""] ; "Sequence of double quotes")]
    fn escaped(s: &str, exp: Vec<&str>) {
        let actual: Vec<_> =
            split_string_unescape(s).map(Result::unwrap).collect();
        assert_eq!(actual, exp);
    }
}
