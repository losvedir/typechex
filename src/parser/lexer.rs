use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Tok {
    LCurly,
    RCurly,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Number(f64),
    Quoted(String),
    Unquoted(String),
    Nil,
    True,
    False,
    Access,
    Kernel,
}

#[derive(Debug, PartialEq)]
pub enum LexicalError {}
pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    c0: Option<char>,
    c1: Option<char>,
    c2: Option<char>,
    c3: Option<char>,
    c4: Option<char>,
    i: usize,
    chars: Chars<'input>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        let mut chars = input.chars();
        let c0 = chars.next();
        let c1 = chars.next();
        let c2 = chars.next();
        let c3 = chars.next();
        let c4 = chars.next();

        Lexer {
            c0: c0,
            c1: c1,
            c2: c2,
            c3: c3,
            c4: c4,
            i: 0,
            chars: chars,
        }
    }

    pub fn shift(&mut self) {
        let c = self.chars.next();
        self.c0 = self.c1;
        self.c1 = self.c2;
        self.c2 = self.c3;
        self.c3 = self.c4;
        self.c4 = c;
        self.i += 1;
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.c0 == None {
                return None;
            }
            if self.c0 == Some('%') && self.c1 == Some('{') && self.c2 == Some('}') {
                for _ in 0..3 {
                    self.shift();
                }
                return Some(Ok((self.i - 3, Tok::Unquoted("%{}".to_string()), self.i)));
            }
            if self.c0 == Some('{') {
                self.shift();
                return Some(Ok((self.i - 1, Tok::LCurly, self.i)));
            }
            if self.c0 == Some('}') {
                self.shift();
                return Some(Ok((self.i - 1, Tok::RCurly, self.i)));
            }
            if self.c0 == Some('[') {
                self.shift();
                return Some(Ok((self.i - 1, Tok::LBrace, self.i)));
            }
            if self.c0 == Some(']') {
                self.shift();
                return Some(Ok((self.i - 1, Tok::RBrace, self.i)));
            }
            if self.c0 == Some(',') {
                self.shift();
                return Some(Ok((self.i - 1, Tok::Comma, self.i)));
            }
            if self.c0 == Some(':') {
                self.shift();
                return Some(Ok((self.i - 1, Tok::Colon, self.i)));
            }

            // Number
            if self.c0.map_or(false, |c| c.is_numeric()) {
                let mut cs: Vec<char> = vec![];
                let i0 = self.i;

                while self
                    .c0
                    .map_or(false, |c| c.is_numeric() || c == '.' || c == 'e')
                {
                    cs.push(self.c0.unwrap());
                    self.shift();
                }

                let num: f64 = cs
                    .iter()
                    .collect::<String>()
                    .parse()
                    .expect("tried to parse not a number");

                return Some(Ok((i0, Tok::Number(num), self.i)));
            }

            // Quoted
            if self.c0 == Some('"') {
                let i0 = self.i;
                let mut cs: Vec<char> = vec![];
                self.shift();
                // TODO: Handle escaped quotes
                while self.c0 != Some('"') {
                    cs.push(self.c0.unwrap());
                    self.shift();
                }
                self.shift();
                let s: String = cs.iter().collect();
                return Some(Ok((i0, Tok::Quoted(s), self.i)));
            }

            // Unquoted
            // TODO: Only allow ?! at end, etc
            if self.c0.map_or(false, |c| is_unquoted(&c)) {
                let i0 = self.i;
                let mut cs: Vec<char> = vec![];
                while self.c0.map_or(false, |c| is_unquoted(&c)) {
                    cs.push(self.c0.unwrap());
                    self.shift();
                }
                let s: String = cs.iter().collect();

                let token = match s.as_ref() {
                    "nil" => Tok::Nil,
                    "true" => Tok::True,
                    "false" => Tok::False,
                    "Access" => Tok::Access,
                    "Kernel" => Tok::Kernel,
                    _ => Tok::Unquoted(s),
                };

                return Some(Ok((i0, token, self.i)));
            }

            self.shift();
        }
    }
}

fn is_unquoted(c: &char) -> bool {
    c.is_alphanumeric()
        || *c == '_'
        || *c == '@'
        || *c == '!'
        || *c == '?'
        || *c == '='
        || *c == '.'
        || *c == '|'
        || *c == '<'
        || *c == '>'
        || *c == '-'
        || *c == '%'
        || *c == '.'
        || *c == '&'
}

#[test]
fn test_lexer() {
    let mut lex: Lexer = Lexer::new("[{:foo, 5}, \"bar\"]");
    assert_eq!(lex.next(), Some(Ok((0, Tok::LBrace, 1))));
    assert_eq!(lex.next(), Some(Ok((1, Tok::LCurly, 2))));
    assert_eq!(lex.next(), Some(Ok((2, Tok::Colon, 3))));
    assert_eq!(
        lex.next(),
        Some(Ok((3, Tok::Unquoted("foo".to_string()), 6)))
    );
    assert_eq!(lex.next(), Some(Ok((6, Tok::Comma, 7))));
    assert_eq!(lex.next(), Some(Ok((8, Tok::Number(5.0), 9))));
    assert_eq!(lex.next(), Some(Ok((9, Tok::RCurly, 10))));
    assert_eq!(lex.next(), Some(Ok((10, Tok::Comma, 11))));
    assert_eq!(
        lex.next(),
        Some(Ok((12, Tok::Quoted("bar".to_string()), 17)))
    );
    assert_eq!(lex.next(), Some(Ok((17, Tok::RBrace, 18))));
}
