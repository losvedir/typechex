pub mod ast;
pub mod lexer;
use crate::error::Error;
use std::process::Command;

lalrpop_mod!(pub elixir, "/parser/elixir.rs");

pub fn parse_file(filename: &str) -> Result<ast::Expr, Error> {
    let contents = dbg!(quote_file(filename)?);
    let lexer = lexer::Lexer::new(&contents);
    let parser = elixir::ExprParser::new();
    Ok(parser.parse(lexer)?)
}

fn quote_file(filename: &str) -> Result<String, Error> {
    let elixir_code = format!(
        "\"{}\" |> File.read!() |> Code.string_to_quoted!() |> IO.inspect()",
        filename
    );

    let output = Command::new("elixir")
        .args(&["-e", &elixir_code])
        .output()?;

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
