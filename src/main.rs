pub mod tokenizer;

use std::process::exit;
use std::{env, fs};

use crate::tokenizer::{Token, TokenType, Tokenizer};

pub struct TokenIterator {
    pub i: i32,
    pub size: usize,
    pub tokens: Vec<Token>,
}

impl TokenIterator {
    pub fn peek(&self) -> Result<Token, ()> {
        return if self.has_next() {
            Ok(self.tokens[self.i as usize + 1])
        } else {
            Err(())
        };
    }

    pub fn curr(&self) -> Result<Token, ()> {
        return if (self.i as usize) < self.size {
            Ok(self.tokens[self.i as usize])
        } else {
            Err(())
        };
    }

    pub fn has_next(&self) -> bool {
        return self.i as usize + 1 < self.size;
    }

    pub fn consume(&mut self) -> Token {
        let t = self.tokens[self.i];
        self.i += 1;
        return t;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing file arg");
        exit(1);
    }

    let contents: String = fs::read_to_string(args.get(1).unwrap()).unwrap();

    let tokens: Vec<Token> = Tokenizer::tokenize(contents);

    dbg!(tokens);
}
