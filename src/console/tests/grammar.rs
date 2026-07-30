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
