use itertools::Itertools;
use std::fmt;

#[derive(Debug, Clone)]
pub struct LexError;

#[derive(Debug, Clone)]
pub enum Operator {
    Add(String),
    Subtract(String),
    Multiply(String),
    Divide(String),
    Assign(String),
}

impl Operator {
    pub fn match_string(value: &str) -> Option<Operator> {
        match value {
            "+" => Some(Operator::Add("+".to_string())),
            "-" => Some(Operator::Subtract("-".to_string())),
            "*" => Some(Operator::Multiply("*".to_string())),
            "/" => Some(Operator::Divide("/".to_string())),
            "=" => Some(Operator::Assign("=".to_string())),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Token {
    IntLiteral(i32),
    Word(String),
    Operator(Operator),
    Stop, // Semicolon
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
}

impl Token {
    pub fn is_int_literal(&self) -> bool {
        match self {
            Token::IntLiteral(_) => true,
            _ => false,
        }
    }
}

pub struct TokenStream<'c> {
    src: &'c str,
    index: usize,
}

impl<'c> TokenStream<'c> {
    pub fn new(src: &'c str) -> Self {
        Self { src, index: 0 }
    }
}

trait TakeInt {
    fn take_int_literal(&mut self) -> Option<Token>;
}

impl<'c> TakeInt for TokenStream<'c> {
    fn take_int_literal(&mut self) -> Option<Token> {
        use std::num::ParseIntError;
        let iter = self.src[self.index..].char_indices();

        let mut raw_val: Vec<(usize, char)> = iter
            .skip_while(|(_, c)| c.is_whitespace())
            .take_while_ref(|(_, c)| c.is_numeric())
            .collect();

        let last_index = raw_val.last().unwrap_or(&(0 as usize, 0 as char)).0;
        let val: String = (&mut raw_val).iter().map(|(_, c)| c).collect();

        if val.len() == 0 {
            return None;
        }

        let val: Result<i32, ParseIntError> = val.parse();

        if val.is_ok() {
            self.index += last_index + 1;
        }

        match val {
            Ok(ival) => Some(Token::IntLiteral(ival)),
            Err(_) => None,
        }
    }
}

trait TakeOperator {
    fn take_operator(&mut self) -> Option<Token>;
}

impl<'c> TakeOperator for TokenStream<'c> {
    fn take_operator(&mut self) -> Option<Token> {
        let mut raw_val: Vec<(usize, char)> = self.src[self.index..]
            .char_indices()
            .skip_while(|(_, c)| c.is_whitespace())
            .take_while(|(_, c)| !c.is_whitespace() && !c.is_alphanumeric())
            .collect();

        let last_index = raw_val.last().unwrap_or(&(0 as usize, 0 as char)).0;
        let val: String = (&mut raw_val).iter().map(|(_, c)| c).collect();

        if val.len() == 0 {
            return None;
        }

        if let Some(op) = Operator::match_string(&val) {
            self.index += last_index + 1;
            return Some(Token::Operator(op));
        }

        None
    }
}

trait TakeStop {
    fn take_stop(&mut self) -> Option<Token>;
}

impl<'c> TakeStop for TokenStream<'c> {
    fn take_stop(&mut self) -> Option<Token> {
        let mut iter = self.src[self.index..]
            .char_indices()
            .skip_while(|(_, c)| c.is_whitespace())
            .peekable();

        if let Some((i, c)) = iter.peek() {
            if *c == ';' {
                self.index += i + 1;
                return Some(Token::Stop);
            }
        }

        None
    }
}

trait TakeWord {
    fn take_word(&mut self) -> Option<Token>;
}

impl<'c> TakeWord for TokenStream<'c> {
    fn take_word(&mut self) -> Option<Token> {
        let mut raw_val: Vec<(usize, char)> = self.src[self.index..]
            .char_indices()
            .skip_while(|(_, c)| c.is_whitespace())
            .take_while_ref(|(_, c)| c.is_alphanumeric() || *c == '_')
            .collect();

        let last_index = raw_val.last().unwrap_or(&(0 as usize, 0 as char)).0;
        let val: String = (&mut raw_val).iter().map(|(_, c)| c).collect();

        if val.len() == 0 {
            return None;
        }

        self.index += last_index + 1;
        Some(Token::Word(val))
    }
}

trait TakePunctuation {
    fn take_punctuation(&mut self) -> Option<Token>;
}

impl<'c> TakePunctuation for TokenStream<'c> {
    fn take_punctuation(&mut self) -> Option<Token> {
        let mut raw_val = self.src[self.index..]
            .char_indices()
            .skip_while(|(_, c)| c.is_whitespace());

        let (i, c) = raw_val.next()?;

        match c {
            '(' => {
                self.index += i + 1;
                Some(Token::OpenParen)
            }
            ')' => {
                self.index += i + 1;
                Some(Token::CloseParen)
            }
            '[' => {
                self.index += i + 1;
                Some(Token::OpenBracket)
            }
            ']' => {
                self.index += i + 1;
                Some(Token::CloseBracket)
            }
            '{' => {
                self.index += i + 1;
                Some(Token::OpenBrace)
            }
            '}' => {
                self.index += i + 1;
                Some(Token::CloseBrace)
            }
            _ => None,
        }
    }
}

impl<'c> Iterator for TokenStream<'c> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.src.len() {
            return None;
        }

        //Lex semicolons.
        let tok = self.take_stop();
        if tok.is_some() {
            return tok;
        }

        // Lex integer literals.
        let tok = self.take_int_literal();
        if tok.is_some() {
            return tok;
        }

        // Lex operators.
        let tok = self.take_operator();
        if tok.is_some() {
            return tok;
        }

        // Lex words.
        let tok = self.take_word();
        if tok.is_some() {
            return tok;
        }

        // Lex punctuation
        let tok = self.take_punctuation();
        if tok.is_some() {
            return tok;
        }

        None
    }
}
