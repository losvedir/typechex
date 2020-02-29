use std::error::Error;
use std::process::Command;

#[cfg(test)]
use std::io::Write;
#[cfg(test)]
use std::process::Stdio;

use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/elixir.pest"]
pub struct ElixirParser;

fn parse_quoted(quoted: &str) -> Result<(), ()> {
    match ElixirParser::parse(Rule::file, quoted) {
        Ok(_res) => Ok(()),
        Err(e) => {
            dbg!(&e);
            Err(())
        }
    }
}

pub fn parse_files(filename: &str) {
    let contents = quote_files(filename).expect("Error from elixir");

    for file in contents.split("@!&*(^)|") {
        if file == "" {
            continue;
        }
        let parts: Vec<&str> = file.split("|)&@^#%").collect();
        println!("{}", parts[0]);

        match parse_quoted(parts[1]) {
            Ok(_exp) => println!("Ok!"),
            Err(_e) => {
                break;
            }
        }
    }
}

fn quote_files(filename: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("elixir")
        .args(&["quote_dir.exs", filename])
        .output()?;

    dbg!(String::from_utf8(output.stderr)?);
    Ok(String::from_utf8(output.stdout)?)
}

#[cfg(test)]
fn quote_string(code: &str) -> Result<String, Box<dyn Error>> {
    let mut child = Command::new("elixir")
        .args(&["quote_str.exs"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(code.as_bytes())
        .expect("Failed to write to stdin");

    let output = child.wait_with_output()?;
    Ok(String::from_utf8(output.stdout)?)
}

#[test]
fn test_quote_string() {
    let quoted = quote_string("%{foo: :bar}").expect("Failed to quote");
    assert_eq!(quoted, "{:%{}, [line: 1], [foo: :bar]}\n");
}

#[test]
fn test_simple_exprs() {
    let exprs = vec![
        ("100000", Rule::integer),
        ("100.0001", Rule::float),
        ("\"I am a binary with spaces and such.\"", Rule::binary),
        ("\"Escaped \\\" within\"", Rule::binary),
        (":foo", Rule::atom),
        (":\"f oo\"", Rule::atom),
        (":{}", Rule::atom),
        ("[]", Rule::list),
        ("[5]", Rule::list),
        ("[5, :foo]", Rule::list),
        ("[foo: 5]", Rule::list),
        ("{}", Rule::tuple),
        ("{5}", Rule::tuple),
        ("{5, :foo}", Rule::tuple),
        ("{:%{}, [line: 1], [foo: 5]}", Rule::tuple),
        ("{:%{}, [line: 11], [path: \"bad_filename\"]}", Rule::tuple),
        ("[\"abcd\\nefgh.\\n\"]", Rule::list),
        (r#"[" \"foo\" "]"#, Rule::list),
    ];

    for (expr, rule) in exprs {
        let parsed = ElixirParser::parse(Rule::expr, expr)
            .expect("Parse error")
            .next()
            .unwrap();

        assert_eq!(parsed.as_rule(), rule);
    }
}
