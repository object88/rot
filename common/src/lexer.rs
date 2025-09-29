use std::{iter::{Enumerate, Peekable}, str::CharIndices};

use crate::tokens::Token;

pub struct Lexer<'a> {
  text: &'a str,
}

pub fn new(text: &str) -> Lexer<'_> {
  Lexer{
    text,
  }
}

pub struct LexerIntoIterator<'a> {
  text: &'a str,

  // itr will return &(usize, (usize, char))
  // char_offset, (char_byte_offset, char)
  itr: Peekable<Enumerate<CharIndices<'a>>>,
}

impl<'a> IntoIterator for Lexer<'a> {
  type Item = Token<'a>;
  type IntoIter = LexerIntoIterator<'a>;

  fn into_iter(self) -> Self::IntoIter {
    LexerIntoIterator { itr: self.text.char_indices().enumerate().peekable(), text: self.text }
  }
}

impl<'a> Iterator for LexerIntoIterator<'a> {
  type Item = Token<'a>;
  
  fn next(&mut self) -> Option<Self::Item> {
    // Chomp intersticial characters
    while let Some(x) = self.itr.peek() {
      match x.1.1 {
        ' ' | '\n' | '\r' | '\t' => _ = self.itr.next(),
        _ => break,
      }
    }

    let x = self.itr.peek();
    match x {
      None => return None,
      Some(x) => {
        let start = x.1.0;
        let c = x.1.1;
        match c {
          // Consume the equal character and return an Assign token
          '=' => {
            _ = self.itr.next();
            return Some(Token::Assign{ position: start });
          },

          '<' => {
            _ = self.itr.next();
            let x0 = self.itr.peek();
            match x0 {
              None => return Some(Token::Illegal),
              Some(x0) => {
                match x0.1.1 {
                  '-' => {
                    _ = self.itr.next();
                    return Some(Token::From { position: start });
                  },
                  _ => {
                    _ = self.itr.next();
                    return Some(Token::Illegal);
                  },
                }
              }
            }
          },

          '(' => {
            _ = self.itr.next();
            return Some(Token::LParen{ position: start });
          },

          ')' => {
            _ = self.itr.next();
            return Some(Token::RParen{ position: start })
          },

          '{' => {
            _ = self.itr.next();
            return Some(Token::LBrace{ position: start })
          },

          '}' => {
            _ = self.itr.next();
            return Some(Token::RBrace{ position: start })
          },

          ':' => {
            _ = self.itr.next();
            return Some(Token::Colon { position: start })
          }

          ',' => {
            _ = self.itr.next();
            return Some(Token::Comma)
          },

          ';' => {
            _ = self.itr.next();
            return Some(Token::Semicolon)
          },

          // Found the beginning of an itentifier
          '_' | 'a'..='z' | 'A'..='Z' => {
            // Consume the first character, remember position of first char
            _ = self.itr.next();

            loop {
              let x0 = self.itr.peek();
              match x0 {
                None => return Some(ident_keyword_check(Token::Ident{ position: start, val: &self.text[start..] })),
                Some(x0) => {
                  match x0.1.1 {
                    '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => _ = self.itr.next(),
                    _ => return Some(ident_keyword_check(Token::Ident{ position: start, val: &self.text[start..x0.1.0] })),
                  }
                }
              }
            }    
          },

          // Found the beginning of a number
          '0'..='9' => {
            // Consume the first character, remember position of first char
            _ = self.itr.next();

            let mut val: i64 = (c as i64) - ('0' as i64);

            loop {
              let x0 = self.itr.peek();
              match x0 {
                None => return Some(Token::Int64{ position: start, val }),
                Some(x0) => {
                  match x0.1.1 {
                    '0'..='9' => {
                      val = (val * 10) + ((x0.1.1 as i64) - ('0' as i64));
                        _ = self.itr.next()
                    },
                    _ => break,
                  }
                }
              }
            }    

            return Some(Token::Int64 { position: start, val })
          },

          // Illegal character
          _ => {
            _ = self.itr.next();
            return Some(Token::Illegal)
          },
        }
      }
     }
  }
}

fn ident_keyword_check<'a>(t: Token<'a>) -> Token<'a> {
  match t {
    Token::Ident { position, val } => {
      match val {
        "fn" => Token::Function { position },
        "let" => Token::Let{ position },
        "proto" => Token::Proto{ position },
        _ => t
      }
    }
    _ => t
  }
}

#[cfg(test)]
mod tests {
  use crate::{lexer, tokens::Token};

  #[test]
  fn test_next_token() {
    let input = "=(){},;";

    let expected = vec![
      Token::Assign{ position: 0 },
      Token::LParen{ position: 1 },
      Token::RParen{ position: 2 },
      Token::LBrace{ position: 3 },
      Token::RBrace{ position: 4 },
      Token::Comma,
      Token::Semicolon
    ];

    let l = lexer::new(input);

    assert_eq!(expected, l.into_iter().collect::<Vec<_>>());
  }

  #[test]
  fn test_code_snippet() {
    let input = "let five = 5;\nlet ten = 10;";

    let expected = vec![
      Token::Let{ position: 0 },
      Token::Ident{ position: 4, val: &"five" },
      Token::Assign{ position: 9 },
      Token::Int64 { position: 11, val: 5 },
      Token::Semicolon,
      Token::Let{ position: 14 },
      Token::Ident{ position: 18, val: &"ten" },
      Token::Assign{ position: 22 },
      Token::Int64 { position: 24, val: 10 },
      Token::Semicolon,
    ];

    let l = lexer::new(input);

    assert_eq!(expected, l.into_iter().collect::<Vec<_>>());
  }

  #[test]
  fn test_proto() {
    let input = "proto unary_math_expression: fn = (out: i64) <- (x: i64)";

    let expected = vec![
      Token::Proto{ position: 0 },
      Token::Ident{ position: 6, val: "unary_math_expression" },
      Token::Colon{ position: 27 },
      Token::Function{ position: 29 },
      Token::Assign { position: 32 },
      Token::LParen{ position: 34 },
      Token::Ident { position: 35, val: "out" },
      Token::Colon { position: 38 },
      Token::Ident { position: 40, val: "i64" },
      Token::RParen { position: 43 },
      Token::From { position: 45 },
      Token::LParen{ position: 48 },
      Token::Ident { position: 49, val: "x" },
      Token::Colon { position: 50 },
      Token::Ident { position: 52, val: "i64" },
      Token::RParen { position: 55 },
    ];

    let l = lexer::new(input);

    assert_eq!(expected, l.into_iter().collect::<Vec<_>>());
  }
}