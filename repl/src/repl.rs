use std::io::{self, BufRead};

use common::lexer;

pub fn run() {
  let stdin = io::stdin();
  print!(">> ");
  for line in stdin.lock().lines() {
    match line {
      Ok(line) => {
        let l = lexer::new(&line);
        for t in l.into_iter() {
          println!("{:?}", t)
        }
      }
      Err(e) => {
        println!("Encountered error: {:?}", e)
      }
    }
  }
}
