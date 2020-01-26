pub mod ast;
pub mod lexer;
use crate::error::Error;
use std::process::Command;

#[cfg(test)]
use std::io::Write;
#[cfg(test)]
use std::process::Stdio;

lalrpop_mod!(pub elixir, "/parser/elixir.rs");

fn parse_quoted(quoted: &str) -> Result<Vec<ast::Expr>, Error> {
    let lexer = lexer::Lexer::new(quoted);
    let parser = elixir::TopParser::new();
    Ok(parser.parse(lexer)?)
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
            Ok(exp) => {
                dbg!(&exp);
                println!("Ok!")
            }
            Err(e) => {
                dbg!(&parts[1]);
                dbg!(e);
                break;
            }
        }
    }
}

fn quote_files(filename: &str) -> Result<String, Error> {
    let output = Command::new("elixir")
        .args(&["quote_dir.exs", filename])
        .output()?;

    dbg!(String::from_utf8(output.stderr)?);
    Ok(String::from_utf8(output.stdout)?)
}

#[cfg(test)]
fn quote_string(code: &str) -> Result<String, Box<dyn std::error::Error>> {
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
fn calculator1() {
    let lexer = lexer::Lexer::new("{}");
    assert_eq!(
        elixir::ExprParser::new().parse(lexer),
        Ok(ast::Expr::Tuple { exprs: vec![] })
    );
}

#[test]
fn test_quote_string() {
    let quoted = quote_string(":foo").expect("Failed to quote");
    assert_eq!(quoted, ":foo\n");
}
