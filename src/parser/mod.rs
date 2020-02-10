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
            dbg!(&quoted);
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
    let quoted = quote_string(":foo").expect("Failed to quote");
    assert_eq!(quoted, ":foo\n");
}

#[test]
fn test_integers() {
    let quoted = quote_string("100_000").expect("Quote error");
    let parsed = ElixirParser::parse(Rule::integer, &quoted)
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::integer);
    assert_eq!(parsed.as_str(), "100000");
}

#[test]
fn test_floats() {
    let quoted = quote_string("100.0001").expect("Quote error");
    let parsed = ElixirParser::parse(Rule::float, &quoted)
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::float);
    assert_eq!(parsed.as_str(), "100.0001");
}

#[test]
fn test_binaries1() {
    let quoted = quote_string("\"I am a binary with spaces and such.\"").expect("Quote error");
    let parsed = ElixirParser::parse(Rule::binary, &quoted)
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::binary);
    assert_eq!(parsed.as_str(), "\"I am a binary with spaces and such.\"")
}

#[test]
fn test_binaries2() {
    let quoted = quote_string("\"Escaped \\\" within\"").expect("Quote error");
    let parsed = ElixirParser::parse(Rule::binary, &quoted)
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::binary);
    assert_eq!(parsed.as_str(), "\"Escaped \\\" within\"");
}

#[test]
fn test_atoms1() {
    let quoted = quote_string(":foo").expect("Quote error");
    let parsed = ElixirParser::parse(Rule::atom, &quoted)
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::atom);
    assert_eq!(parsed.as_str(), ":foo")
}

#[test]
fn test_atoms2() {
    let quoted = quote_string(":\"f oo\"").expect("Quote error");
    let parsed = ElixirParser::parse(Rule::atom, &quoted)
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::atom);
    assert_eq!(parsed.as_str(), ":\"f oo\"")
}

#[test]
fn test_atoms3() {
    let quoted = quote_string(":{}").expect("Quote error");
    let parsed = ElixirParser::parse(Rule::atom, &quoted)
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::atom);
    assert_eq!(parsed.as_str(), ":{}")
}

#[test]
fn test_atoms4() {
    let parsed = ElixirParser::parse(Rule::atom, ":%{}")
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::atom);
    assert_eq!(parsed.as_str(), ":%{}")
}

#[test]
fn test_list_empty() {
    let quoted = quote_string("[]").expect("Quote error");
    let parsed = ElixirParser::parse(Rule::list, &quoted)
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::list);
    assert_eq!(parsed.as_str(), "[]")
}

#[test]
fn test_list_one_element() {
    let quoted = quote_string("[5]").expect("Quote error");
    let parsed = ElixirParser::parse(Rule::list, &quoted)
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::list);
    assert_eq!(parsed.as_str(), "[5]");
    assert_eq!(parsed.into_inner().next().unwrap().as_rule(), Rule::integer);
}

#[test]
fn test_list_multiple_elements() {
    let quoted = quote_string("[5, :foo]").expect("Quote error");
    let parsed = ElixirParser::parse(Rule::list, &quoted)
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::list);
    assert_eq!(parsed.as_str(), "[5, :foo]");
    let mut inner = parsed.into_inner();
    assert_eq!(inner.next().unwrap().as_rule(), Rule::integer);
    assert_eq!(inner.next().unwrap().as_rule(), Rule::atom);
}

#[test]
fn test_list_sugar_tuples() {
    let parsed = ElixirParser::parse(Rule::list, "[foo: 5]")
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::list);
    assert_eq!(parsed.as_str(), "[foo: 5]");
    let mut inner = parsed.into_inner();
    assert_eq!(inner.next().unwrap().as_rule(), Rule::sugar_tuple);
}

#[test]
fn test_tuple_empty() {
    let parsed = ElixirParser::parse(Rule::tuple, "{}")
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::tuple);
    assert_eq!(parsed.as_str(), "{}")
}

#[test]
fn test_tuple_one_element() {
    let parsed = ElixirParser::parse(Rule::tuple, "{5}")
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::tuple);
    assert_eq!(parsed.as_str(), "{5}");
    assert_eq!(parsed.into_inner().next().unwrap().as_rule(), Rule::integer);
}

#[test]
fn test_tuple_multiple_elements() {
    let parsed = ElixirParser::parse(Rule::tuple, "{5, :foo}")
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::tuple);
    assert_eq!(parsed.as_str(), "{5, :foo}");
    let mut inner = parsed.into_inner();
    assert_eq!(inner.next().unwrap().as_rule(), Rule::integer);
    assert_eq!(inner.next().unwrap().as_rule(), Rule::atom);
}

#[test]
fn test_map() {
    let quoted = quote_string("%{foo: 5}").expect("Quote error");
    let parsed = ElixirParser::parse(Rule::expr, &quoted)
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::tuple);
    assert_eq!(parsed.as_str(), "{:%{}, [line: 1], [foo: 5]}");
}

#[test]
fn test_failure() {
    let parsed = ElixirParser::parse(Rule::expr, "{:%{}, [line: 11], [path: \"bad_filename\"]}")
        .expect("Parse error")
        .next()
        .unwrap();
    assert_eq!(parsed.as_rule(), Rule::tuple);
}
