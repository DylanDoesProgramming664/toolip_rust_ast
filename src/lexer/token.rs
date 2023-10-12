#![allow(unused)]
use std::{collections::HashMap, ops::Index};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Token {
    Symbol(Symbol),
    KeyWord(KeyWord),
    Identifier(String),
    VarDef(String),
    Literal(Literal),
    NewLine,
    EOF,
    Illegal,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Symbol {
    Plus,
    // Minus,
    // Star,
    // Carrot,
    // Slash,
    // Percent,
    LessThan,
    Equals,
    GreaterThan,
    // And,
    // Or,
    // Not,
    // Bang,
    // DoubleDot,
    // TripleDot,
    // DoubleQuestion,
    // BitShiftLeft,
    // BitShiftRight,
    // DoublePlus,
    // DoubleMinus,
    // DoubleSlash,
    // DoubleEquals,
    // PlusEquals,
    // MinusEquals,
    // StarEquals,
    // CarrotEquals,
    // SlashEquals,
    // PercentEquals,
    // LessThanEquals,
    // GreaterThanEquals,
    // AndEquals,
    // OrEquals,
    // BangEquals,
    // DoubleDotEquals,
    // TripleDotEquals,
    // DoubleQuestionEquals,
    // BitShiftLeftEquals,
    // BitShiftRightEquals,
    // DoubleSlashEquals,
    // AtSign,
    // Dot,
    Comma,
    OpenParen,
    CloseParen,
    // OpenBracket,
    // CloseBracket,
    // OpenBrace,
    // CloseBrace,
    // SemiColon,
    // Colon,
    Invalid,
}

impl Into<usize> for Symbol {
    fn into(self) -> usize {
        return match self {
            Self::Plus => 0,
            // Self::Minus => 1,
            // Self::Star => 2,
            // Self::Carrot => 3,
            // Self::Slash => 4,
            // Self::Percent => 5,
            Self::LessThan => 6,
            Self::Equals => 7,
            Self::GreaterThan => 8,
            // Self::And => 9,
            // Self::Or => 10,
            // Self::Not => 11,
            // Self::Bang => 12,
            // Self::DoubleDot => 13,
            // Self::TripleDot => 14,
            // Self::DoubleQuestion => 15,
            // Self::BitShiftLeft => 16,
            // Self::BitShiftRight => 17,
            // Self::DoublePlus => 18,
            // Self::DoubleMinus => 19,
            // Self::DoubleSlash => 20,
            // Self::DoubleEquals => 21,
            // Self::PlusEquals => 22,
            // Self::MinusEquals => 23,
            // Self::StarEquals => 24,
            // Self::CarrotEquals => 25,
            // Self::SlashEquals => 26,
            // Self::PercentEquals => 27,
            // Self::LessThanEquals => 28,
            // Self::GreaterThanEquals => 29,
            // Self::AndEquals => 30,
            // Self::OrEquals => 31,
            // Self::BangEquals => 32,
            // Self::DoubleDotEquals => 33,
            // Self::TripleDotEquals => 34,
            // Self::DoubleQuestionEquals => 35,
            // Self::BitShiftLeftEquals => 36,
            // Self::BitShiftRightEquals => 37,
            // Self::DoubleSlashEquals => 38,
            // Self::AtSign => 39,
            // Self::Dot => 40,
            Self::Comma => 41,
            Self::OpenParen => 42,
            Self::CloseParen => 43,
            // Self::OpenBracket => 44,
            // Self::CloseBracket => 45,
            // Self::OpenBrace => 46,
            // Self::CloseBrace => 47,
            // Self::SemiColon => 48,
            // Self::Colon => 49,
            Self::Invalid => 50,
        };
    }
}

impl From<usize> for Symbol {
    fn from(index: usize) -> Self {
        return match index {
            0 => Self::Plus,
            // Self::Minus => 1,
            // Self::Star => 2,
            // Self::Carrot => 3,
            // Self::Slash => 4,
            // Self::Percent => 5,
            6 => Self::LessThan,
            7 => Self::Equals,
            8 => Self::GreaterThan,
            // Self::And => 9,
            // Self::Or => 10,
            // Self::Not => 11,
            // Self::Bang => 12,
            // Self::DoubleDot => 13,
            // Self::TripleDot => 14,
            // Self::DoubleQuestion => 15,
            // Self::BitShiftLeft => 16,
            // Self::BitShiftRight => 17,
            // Self::DoublePlus => 18,
            // Self::DoubleMinus => 19,
            // Self::DoubleSlash => 20,
            // Self::DoubleEquals => 21,
            // Self::PlusEquals => 22,
            // Self::MinusEquals => 23,
            // Self::StarEquals => 24,
            // Self::CarrotEquals => 25,
            // Self::SlashEquals => 26,
            // Self::PercentEquals => 27,
            // Self::LessThanEquals => 28,
            // Self::GreaterThanEquals => 29,
            // Self::AndEquals => 30,
            // Self::OrEquals => 31,
            // Self::BangEquals => 32,
            // Self::DoubleDotEquals => 33,
            // Self::TripleDotEquals => 34,
            // Self::DoubleQuestionEquals => 35,
            // Self::BitShiftLeftEquals => 36,
            // Self::BitShiftRightEquals => 37,
            // Self::DoubleSlashEquals => 38,
            // Self::AtSign => 39,
            // Self::Dot => 40,
            41 => Self::Comma,
            42 => Self::OpenParen,
            43 => Self::CloseParen,
            // Self::OpenBracket => 44,
            // Self::CloseBracket => 45,
            // Self::OpenBrace => 46,
            // Self::CloseBrace => 47,
            // Self::SemiColon => 48,
            // Self::Colon => 49,
            _ => Self::Invalid,
        };
    }
}

pub const SYMBOLS: [&str; 50] = [
    "+", "-", "*", "^", "/", "%", "<", "=", ">", "&", "|", "~", "!", "..", "...", "??", "<<", ">>",
    "++", "--", "//", "==", "+=", "-=", "*=", "^=", "/=", "%=", "<=", ">=", "&=", "|=", "!=",
    "..=", "...=", "??=", "<<=", ">>=", "//=", "@", ".", ",", "(", ")", "[", "]", "{", "}", ";",
    ":",
];

impl Symbol {
    pub fn get_symbol_value(&self) -> &str {
        let index: usize = (*self).into();
        return SYMBOLS[index];
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum KeyWord {
    // Open,
    // Petal,
    // Global,
    // Const,
    // Static,
    // Jit,
    // Gc,  // Garbage collection memory model
    // Alc, // Allocator memory model
    // Ob,  // Owner-borrow memory model
    // Coroutine,
    // If,
    // Else,
    // Then,
    End,
    // While,
    // For,
    // Loop,
    // Do,
    // Break,
    // Continue,
    // Match,
    // Type, // Type
    // Enum,
    Function,
    Return,
    // Struct,
    // Union,
    // Trait,
    // Modules,
    // Selfy,         // Self
    // SelfLowerCase, // self
    Invalid,
}

impl From<usize> for KeyWord {
    fn from(index: usize) -> Self {
        return match index {
            // 0 => Self::Open,
            // 1 => Self::Petal,
            // 2 => Self::Global,
            // 3 => Self::Const,
            // 4 => Self::Static,
            // 5 => Self::Jit,
            // 6 => Self::Gc,
            // 7 => Self::Alc,
            // 8 => Self::Ob,
            // 9 => Self::Coroutine,
            // 10 => Self::If,
            // 11 => Self::Else,
            // 12 => Self::Then,
            13 => Self::End,
            // 14 => Self::While,
            // 15 => Self::For,
            // 16 => Self::Loop,
            // 17 => Self::Do,
            // 18 => Self::Break,
            // 19 => Self::Continue,
            // 20 => Self::Match,
            // 21 => Self::Type,
            // 22 => Self::Enum,
            23 => Self::Function,
            24 => Self::Return,
            // 25 => Self::Struct,
            // 26 => Self::Union,
            // 27 => Self::Trait,
            // 28 => Self::Modules,
            // 29 => Self::Selfy,
            // 30 => Self::SelfLowerCase,
            _ => Self::Invalid,
        };
    }
}

pub const KEYWORDS: [&'static str; 31] = [
    "open",
    "petal",
    "glob",
    "const",
    "static",
    "jit",
    "gc",
    "alc",
    "ob",
    "coroutine",
    "if",
    "else",
    "then",
    "end",
    "while",
    "for",
    "loop",
    "do",
    "break",
    "continue",
    "match",
    "type",
    "enum",
    "func",
    "return",
    "struct",
    "union",
    "trait",
    "mod",
    "Self",
    "self",
];

pub fn match_keyword(token_value: &'static str) -> Token {
    for (index, keyword) in KEYWORDS.into_iter().enumerate() {
        if token_value == keyword {
            return Token::KeyWord(KeyWord::from(index));
        }
    }
    return Token::Illegal;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Literal {
    Uint(usize),
}
