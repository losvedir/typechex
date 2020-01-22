pub mod ast;
pub mod lexer;
use crate::error::Error;
use std::process::Command;

lalrpop_mod!(pub elixir, "/parser/elixir.rs");

pub fn parse_file(filename: &str) {
    let contents = quote_file(filename).expect("Error from elixir");

    for file in contents.split("@!&*(^)|") {
        if file == "" {
            continue;
        }
        let parts: Vec<&str> = file.split("|)&@^#%").collect();
        println!("{}", parts[0]);

        let lexer = lexer::Lexer::new(&parts[1]);
        let parser = elixir::TopParser::new();

        match parser.parse(lexer) {
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

fn quote_file(filename: &str) -> Result<String, Error> {
    // let elixir_code = format!(
    //     "\"{}\" |> File.read!() |> Code.string_to_quoted!() |> IO.inspect(limit: :infinity)",
    //     filename
    // );
    //
    // let output = Command::new("elixir")
    //     .args(&["-e", &elixir_code])
    //     .output()?;

    let output = Command::new("elixir")
        .args(&["quoter.exs", filename])
        .output()?;

    dbg!(String::from_utf8(output.stderr)?);
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
