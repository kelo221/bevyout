use super::ConsoleError;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ParsedCommand {
    pub(crate) reference: Option<String>,
    pub(crate) name: String,
    pub(crate) args: Vec<String>,
}

pub(crate) fn parse_command(line: &str) -> Result<Option<ParsedCommand>, ConsoleError> {
    let tokens = tokenize(line)?;
    let Some(head) = tokens.first() else {
        return Ok(None);
    };

    let (reference, name) = match head.rsplit_once('.') {
        Some((reference, name)) if !reference.is_empty() && !name.is_empty() => {
            (Some(reference.to_string()), name.to_ascii_lowercase())
        }
        _ => (None, head.to_ascii_lowercase()),
    };
    if !name
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        return Err(ConsoleError::new(
            "invalid_command",
            "command names may contain only letters, numbers, and '_'",
        ));
    }

    Ok(Some(ParsedCommand {
        reference,
        name,
        args: tokens.into_iter().skip(1).collect(),
    }))
}

fn tokenize(line: &str) -> Result<Vec<String>, ConsoleError> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = line.chars().peekable();
    let mut quoted = false;

    while let Some(character) = chars.next() {
        if quoted {
            match character {
                '"' => quoted = false,
                '\\' => match chars.next() {
                    Some('"') => current.push('"'),
                    Some('\\') => current.push('\\'),
                    Some(other) => {
                        current.push('\\');
                        current.push(other);
                    }
                    None => current.push('\\'),
                },
                other => current.push(other),
            }
            continue;
        }

        match character {
            ';' => break,
            '"' => quoted = true,
            value if value.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            other => current.push(other),
        }
    }

    if quoted {
        return Err(ConsoleError::new(
            "unterminated_quote",
            "quoted argument is missing its closing quote",
        ));
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_gamebryo_command_table() {
        let cases = [
            (
                "player.moveto ff0000a1",
                Some("player"),
                "moveto",
                vec!["ff0000a1"],
            ),
            ("prid 0001a2b3", None, "prid", vec!["0001a2b3"]),
            ("setpos z 1.5", None, "setpos", vec!["z", "1.5"]),
            ("HELP \"set pos\" ; ignored", None, "help", vec!["set pos"]),
        ];

        for (line, reference, name, args) in cases {
            let parsed = parse_command(line).unwrap().unwrap();
            assert_eq!(parsed.reference.as_deref(), reference);
            assert_eq!(parsed.name, name);
            assert_eq!(parsed.args, args);
        }
        assert_eq!(parse_command(" ; comment").unwrap(), None);
        assert_eq!(parse_command("   ").unwrap(), None);
    }

    #[test]
    fn reports_unterminated_quotes() {
        let error = parse_command("help \"nope").unwrap_err();
        assert_eq!(error.code, "unterminated_quote");
    }
}
