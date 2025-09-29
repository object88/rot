

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
  Illegal,
  EOF,

  Ident { position: usize, val: &'a str },
  Int64 { position: usize, val: i64 },

  // Operators

  // `=`
  Assign { position: usize },
  // `<-`
  From { position: usize },

  // Delimiters
  Colon { position: usize },
  Comma,
  Semicolon,

  // `(`
  LParen { position: usize },

  // `)`
  RParen { position: usize },

  // `{`
  LBrace { position: usize },

  // `}`
  RBrace { position: usize },

  // `[`
  LBracket { position: usize },

  // `]`
  RBracket { position: usize },

  // Keywords
  Const { position: usize },
  Function { position: usize },
  Let { position: usize },
  Proto { position: usize },
}