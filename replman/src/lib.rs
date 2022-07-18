use std::str::FromStr;

use parsing::split_string_unescape;
use rustyline::Editor;

mod parsing;
mod repl;
mod replman;

pub mod prelude {
    pub use replman_derive::Replman;

    pub use crate::repl::Repl;
    pub use crate::{read_command, Replman};
}

/// A trait representing a set of commands with all the REPL related utilities provided
pub trait Replman {
    /// Displays the help string for all commands
    fn help() -> &'static str;

    /// Parses the command from an iterator over parts of a string
    fn parse<'a, I>(parts: I) -> anyhow::Result<Self>
    where
        Self: Sized,
        I: Iterator<Item = anyhow::Result<&'a str>> + 'a;

    /// Parses the command from a string
    fn parse_str(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Self::parse(split_string_unescape(s))
    }
}

/// A trait used for parsing repl command arguments
pub trait ReplmanParse {
    /// Parses from an iterator item, accepts Option<&str> so that optional fields can "parse" from None
    fn parse(item: Option<&str>) -> anyhow::Result<Self>
    where
        Self: Sized;

    /// Parses a string, used when the value of the cmd is guaranteed to be present (e.g. with #[replman(default)])
    fn parse_default(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized;
}

macro_rules! impl_with_from_str {
    ($t:ty) => {
        impl ReplmanParse for $t {
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

impl<T> ReplmanParse for Option<T>
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
    R: Replman,
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
