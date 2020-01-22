#[derive(Debug, PartialEq)]
pub enum Expr {
    Tuple { exprs: Vec<Expr> },
    List { exprs: Vec<Expr> },
    Atom(String),
    Binary(String),
    Number(f64),
    Defmodule { exprs: Vec<Expr> },
}
