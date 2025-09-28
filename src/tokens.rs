

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
  Illegal,
  EOF,

  Ident { position: usize, val: &'a str },
  Int64 { position: usize, val: i64 },

  // Operators
  Assign { position: usize },
  Plus { position: usize },

  // Delimiters
  Comma,
  Semicolon,

  LParen,
  RParen,
  LBrace,
  RBrace,

  // Keywords
  Function,
  Let { position: usize },
}