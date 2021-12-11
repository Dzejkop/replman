use std::io::{stdin, stdout, Write};
use std::str::FromStr;

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
