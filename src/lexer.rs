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

          // Consume the plus character and return an Assign token
          '+' => {
            _ = self.itr.next();
            return Some(Token::Plus{ position: start });
          },

          '(' => {
            _ = self.itr.next();
            return Some(Token::LParen);
          },

          ')' => {
            _ = self.itr.next();
            return Some(Token::RParen)
          },

          '{' => {
            _ = self.itr.next();
            return Some(Token::LBrace)
          },

          '}' => {
            _ = self.itr.next();
            return Some(Token::RBrace)
          },

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
        "let" => Token::Let{ position },
        _ => t
      }
    }
    _ => t
  }
}

#[cfg(test)]
mod tests {
  use crate::{lexer, tokens};

  #[test]
  fn test_next_token() {
    let input = "=+(){},;";

    let expected = vec![
      tokens::Token::Assign{ position: 0 },
      tokens::Token::Plus{ position: 1 },
      tokens::Token::LParen,
      tokens::Token::RParen,
      tokens::Token::LBrace,
      tokens::Token::RBrace,
      tokens::Token::Comma,
      tokens::Token::Semicolon
    ];

    let l = lexer::new(input);

    assert_eq!(expected, l.into_iter().collect::<Vec<_>>());
  }

  #[test]
  fn test_code_snippet() {
    let input = "let five = 5;\nlet ten = 10;";

    let expected = vec![
      tokens::Token::Let{ position: 0 },
      tokens::Token::Ident{ position: 4, val: &"five" },
      tokens::Token::Assign{ position: 9 },
      tokens::Token::Int64 { position: 11, val: 5 },
      tokens::Token::Semicolon,
      tokens::Token::Let{ position: 14 },
      tokens::Token::Ident{ position: 18, val: &"ten" },
      tokens::Token::Assign{ position: 22 },
      tokens::Token::Int64 { position: 24, val: 10 },
      tokens::Token::Semicolon,
    ];

    let l = lexer::new(input);

    assert_eq!(expected, l.into_iter().collect::<Vec<_>>());
  }
}