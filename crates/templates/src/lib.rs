#![feature(trivial_bounds)]
use serde::{Deserialize, Serialize};
use serde_xml_rs::to_string;
use std::any::Any;
use std::io::Read;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenTag(String),               // <tag
    CloseTag(String),              // </tag>
    SelfClosingTag(String),        // <tag/>
    EndTag,                        // >
    Equals,                        // =
    Text(String),                  // text content
    QuotedString(String),          // "string" or 'string'
    Identifier(String),            // attribute names
    Comment(String),               // <!-- comment -->
    ProcessingInstruction(String), // <?xml ...?>
    Attr {
        name: String,
        // serialized in ron
        value: String,
    },
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            position: 0,
        }
    }
    fn advance(&mut self) -> Option<char> {
        self.position += 1;
        self.input.next()
    }

    fn dbg(&self) {
        dbg!(&self.input);
        dbg!(self.position);
    }
    fn read_quoted_string(&mut self, quote_char: char) -> Token {
        let mut string = String::new();
        while let Some(c) = self.advance() {
            if c == quote_char {
                return Token::QuotedString(string);
            }
            string.push(c);
        }
        // Handle unclosed quotes as an error in production code
        Token::QuotedString(string)
    }
    fn read_identifier(&mut self, first_char: char) -> String {
        let mut identifier = String::new();
        identifier.push(first_char);

        while let Some(&c) = self.input.peek() {
            if !c.is_alphanumeric() && c != '_' && c != '-' && c != ':' {
                break;
            }
            identifier.push(self.advance().unwrap());
        }
        identifier
    }

    fn next_token(&mut self) -> Option<Token> {
        let token = self.input.next()?;
        match token {
            '<' => {
                let mut identifier = String::new();
                identifier.push(token);

                while let Some(&c) = self.input.peek() {
                    if !c.is_alphanumeric() && c != '_' && c != '-' && c != ':' {
                        break;
                    }
                    self.position += 1;
                    identifier.push(self.input.next().unwrap());
                }

                Some(Token::OpenTag(identifier))
            }

            '>' => Some(Token::EndTag),
            '=' => Some(Token::Equals),
            '"' => Some(self.read_quoted_string(token)),

            _ => {
                if token.is_alphabetic() || token == '_' {
                    Some(Token::Identifier(self.read_identifier(token)))
                } else {
                    let mut text = String::new();
                    text.push(token);
                    while let Some(&c) = self.input.peek() {
                        if c == '<' || c == '>' || c == '=' || c == '"' || c == '\'' {
                            break;
                        }
                        text.push(self.advance().unwrap());
                    }
                    Some(Token::Text(text))
                }
            }
        }
    }
}

fn process_tokens(tokens: Vec<Token>) -> Vec<Token> {
    let mut processed: Vec<Token> = Vec::new();
    let mut idx = 0;

    let _v: &str = "<";
    while idx < tokens.len() {
        match tokens.get(idx) {
            Some(Token::OpenTag(_v)) => {
                // Check for end tags
                if let (Some(Token::Text(text)), Some(Token::EndTag)) =
                    (tokens.get(idx + 1), tokens.get(idx + 2))
                {
                    if text.starts_with('/') {
                        processed.push(Token::CloseTag(text[1..].to_string()));
                        idx += 3;
                        continue;
                    }
                }

                processed.push(tokens[idx].clone());
                idx += 1;
            }
            Some(Token::Text(name)) => {
                // Check for attributes
                if let (Some(Token::Equals), Some(Token::QuotedString(value))) =
                    (tokens.get(idx + 1), tokens.get(idx + 2))
                {
                    processed.push(Token::Attr {
                        name: name.trim().to_string(),
                        value: value.to_string(),
                    });
                    idx += 3;
                } else {
                    processed.push(Token::Text(name.clone()));
                    idx += 1;
                }
            }
            Some(token) => {
                processed.push(token.clone());
                idx += 1;
            }
            None => break,
        }
    }

    processed
}



#[test]
fn test() {
}
