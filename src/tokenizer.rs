use crate::{
    consts::KEYWORDS,
    operators::{BinaryOperator, BINARY_OPERATORS},
    token::Token,
};

pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(inp: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut iter = TokenizerIterator {
            i: 0,
            s: 0,
            len: inp.len(),
            content: inp.chars().collect(),
        };

        while !iter.end() {
            match iter.word() {
                Some(word) => {
                    if KEYWORDS.contains(&word.as_str()) {
                        tokens.push(Token::Keyword(word));
                    } else {
                        tokens.push(Token::Identifier(word));
                    }
                }
                None => {
                    match iter.curr() {
                        Some('"') => {
                            tokens.push(Token::StrLiteral(iter.until('"')));
                        }
                        Some(';') => {
                            tokens.push(Token::Semi);
                        }
                        Some(d) => {
                            if d.is_digit(10) {
                                let mut val = String::new();
                                val.push(d);
                                while iter.peek().map_or(false, |p| p.is_digit(10)) {
                                    if let Some(next) = iter.next() {
                                        val.push(next);
                                    }
                                }
                                tokens.push(Token::IntLiteral(val));
                            } else {
                                for bin in BINARY_OPERATORS {
                                    if iter.match_skip(bin.op) {
                                        tokens.push(Token::Binary(BinaryOperator {
                                            op: bin.op,
                                            precedence: bin.precedence,
                                        }));
                                        break;
                                    }
                                }
                            }
                        }
                        None => panic!("Unknown char at {}", iter.i),
                    }
                    iter.next();
                }
            }
        }
        return tokens;
    }
}

#[derive(Debug)]
pub struct TokenizerIterator {
    pub i: usize,
    pub s: usize,
    pub len: usize,
    pub content: Vec<char>,
}

impl TokenizerIterator {
    pub fn end(&self) -> bool {
        return self.i >= self.len;
    }

    pub fn curr(&self) -> Option<char> {
        return if self.i < self.len {
            Some(self.content[self.i])
        } else {
            None
        };
    }

    pub fn next(&mut self) -> Option<char> {
        self.i += 1;
        self.s = self.i;
        return if self.i < self.len {
            Some(self.content[self.i])
        } else {
            None
        };
    }

    pub fn peek(&mut self) -> Option<char> {
        self.s += 1;
        return if self.s < self.len {
            Some(self.content[self.s])
        } else {
            None
        };
    }

    pub fn consume(&mut self) {
        self.i = self.s;
    }

    pub fn back(&mut self) -> char {
        if self.i > 0 {
            self.i -= 1;
        }
        self.s = self.i;
        return self.content[self.i];
    }

    pub fn matches(&self, str: &str) -> bool {
        let chars: Vec<char> = str.chars().collect();
        for i in 0..str.len() {
            if self.content[self.i + i] != chars[i] {
                return false;
            }
        }
        return true;
    }

    pub fn match_skip(&mut self, str: &str) -> bool {
        if self.matches(str) {
            self.i += str.len();
            self.s = self.i;
            return true;
        }
        return false;
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(curr) = self.curr() {
            if curr.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    pub fn word(&mut self) -> Option<String> {
        let mut out: String = String::new();

        self.skip_whitespace();

        if self.curr()?.is_alphabetic() {
            out.push(self.curr()?);
            while self.next()?.is_alphanumeric() {
                out.push(self.curr()?);
            }
            return Some(out);
        }
        return None;
    }

    pub fn until(&mut self, stop: char) -> String {
        let mut out = String::new();
        while let Some(c) = self.next() {
            if c == stop {
                break;
            } else {
                out.push(c);
            }
        }
        return out;
    }
}
