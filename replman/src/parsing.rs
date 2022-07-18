pub fn split_string_unescape(
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
