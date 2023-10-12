#![allow(unused)]
use token::{match_keyword, Literal, Symbol, Token, KEYWORDS};

pub mod token;

#[derive(Debug, Clone)]
pub struct Lexer {
    input: Vec<char>,
    char: char,
    prev_char: char,
    pos: usize,
    next_pos: usize,
    // line_num: usize,
    // line_pos: usize,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        let mut lexer = Self {
            input,
            char: '\0',
            prev_char: '\0',
            pos: 0,
            next_pos: 0,
            // line_num: 1,
            // line_pos: 0,
        };

        lexer.read_char();

        return lexer;
    }

    fn read_char(&mut self) {
        let input_len = self.input.len();
        self.prev_char = self.char;
        self.char = if self.next_pos >= input_len {
            '\0'
        } else {
            self.input[self.next_pos]
        };
        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    fn next_char(&self) -> char {
        let input_len = self.input.len();
        return if self.next_pos >= input_len {
            '\0'
        } else {
            self.input[self.next_pos]
        };
    }

    fn next_nth_char(&self, peek_num: usize) -> char {
        let input_len = self.input.len();
        let peek_pos = self.pos + peek_num;
        return if peek_pos >= input_len {
            '\0'
        } else {
            self.input[peek_pos]
        };
    }

    fn is_next_char(&self, char: char) -> bool {
        return self.next_char() == char;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.char {
            '+' => Token::Symbol(Symbol::Plus),
            '<' => Token::Symbol(Symbol::LessThan),
            '=' => Token::Symbol(Symbol::Equals),
            '>' => Token::Symbol(Symbol::GreaterThan),
            ',' => Token::Symbol(Symbol::Comma),
            '(' => Token::Symbol(Symbol::OpenParen),
            ')' => Token::Symbol(Symbol::CloseParen),
            '\n' => Token::NewLine,
            '\0' => Token::EOF,
            x => match x {
                '0'..='9' => self.read_number(),
                'a'..='z' | 'A'..='Z' => self.read_identifier(),
                _ => Token::Illegal,
            },
        };

        self.read_char();

        println!("{:?}", token);

        return token;
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.char {
                ' ' | '\t' => {
                    self.read_char();
                }
                _ => break,
            }
        }
    }

    fn read_number(&mut self) -> Token {
        let mut number = String::new();

        //Might be off by one
        while self.next_char().is_digit(10) {
            number.push(self.char);
            self.read_char();
        }

        return Token::Literal(Literal::Uint(number.parse().unwrap()));
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        //Might be off by one
        while self.next_char().is_alphanumeric() || self.next_char() == '_' {
            identifier.push(self.char);
            self.read_char();
        }

        return match_identifier(identifier);
    }
}

fn match_identifier(token_value: String) -> Token {
    return if KEYWORDS.contains(&token_value.as_str()) {
        match_keyword(token_value.)
    } else {
        Token::Identifier(token_value.to_owned())
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        let input: Vec<char> = "+<=>,()".chars().collect();

        let exp_tokens: Vec<Token> = vec![
            Token::Symbol(Symbol::Plus),
            Token::Symbol(Symbol::LessThan),
            Token::Symbol(Symbol::Equals),
            Token::Symbol(Symbol::GreaterThan),
            Token::Symbol(Symbol::Comma),
            Token::Symbol(Symbol::OpenParen),
            Token::Symbol(Symbol::CloseParen),
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        for exp_token in exp_tokens.iter() {
            let token = lexer.next_token();
            assert_eq!(token, *exp_token);
        }
    }
}
