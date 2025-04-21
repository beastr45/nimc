use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{Context, Result};
use log::{debug, error, info, trace, warn};
use colored::Colorize;
use regex::Regex;

// Here are the regex strings:
// Identifier [a-zA-Z_]\w*\b
// Constant [0-9]+\b

// TODO: consider making a tokenizer struct for lex() to drive

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    // Or also Literal
    Constant(i32),
    Keywords(Keyword),
    Punctuators(String),
    // Symbols (Punctuators)
    // OpenParen,
    // CloseParen,
    // OpenBrace,
    // CloseBrace,
    // Semicolon,
    //Operators
}

//Keywords
#[derive(Debug)]
pub enum Keyword {
    Int,
    Return,
}

enum TokenType {
    Identifier,
    Number,
    Punctuator,
    Whitespace,
    Word,
    Other,
}

struct Tokenizer {
    regexes: Vec<(Regex, TokenType)>,
}

impl Tokenizer {
    fn new() -> Self {
        // 1. Fixed regex patterns with proper ordering and anchoring
        let regexes = vec![
            (Regex::new(r"^\s+").unwrap(), TokenType::Whitespace), // Whitespace (skip)
            (
                Regex::new(r"^[a-zA-Z_]\w*\b").unwrap(),
                TokenType::Identifier,
            ),
            (Regex::new(r"^[0-9]+\b").unwrap(), TokenType::Number),
            (
                Regex::new(r"^(\(|\)|\{|\}|;)").unwrap(),
                TokenType::Punctuator,
            ), // Single-character punctuators
        ];
        Tokenizer { regexes }
    }
    fn lex_line(&self, line: &str) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut remaining = line.trim_start();

        while !remaining.is_empty() {
            let mut matched = false;
            for (regex, token_type) in &self.regexes {
                if let Some(mat) = regex.find(remaining) {
                    let token_str = mat.as_str();
                    let token = match token_type {
                        TokenType::Identifier => self.match_identifier(token_str),
                        TokenType::Number => Token::Constant(token_str.parse::<i32>()?),
                        TokenType::Punctuator => Token::Punctuators(token_str.to_string()),
                        _ => {
                            remaining = &remaining[mat.end()..];
                            matched = true;
                            break;
                        }
                    };
                    debug!("Matched '{}' with '{}'", mat.as_str(), regex.as_str());
                    tokens.push(token);
                    remaining = &remaining[mat.end()..];
                    matched = true;
                    break;
                }
            }
            //TODO: implement the custom error types
            if !matched {
                anyhow::bail!("Invalid tokens in line:{}",line.red());
            }
        }
        Ok(tokens)
    }

    fn match_identifier(&self, s: &str) -> Token {
        // huge match statement here w/ strings
        // will return an Identifier keyword token type based on Identifier contents
        // otherwise token type with Identifier string value
        match s {
            "int" => Token::Keywords(Keyword::Int),
            "return" => Token::Keywords(Keyword::Return),
            _ => Token::Identifier(s.to_string()),
        }
    }
}

pub fn lex(input_path: &Path) -> Result<Vec<Token>> {
    // drive the tokenizer struct here

    let file = File::open(input_path)
        .with_context(|| format!("Failed to open file : {}", input_path.display()))?;
    let freader = BufReader::new(file);

    let tokenizer = Tokenizer::new();

    let mut tokens = Vec::<Token>::new();

    for (line_num, line) in freader.lines().enumerate() {
        let line = line?;
        let line_tokens = tokenizer
            .lex_line(&line)
            .with_context(|| format!("Error lexing line {}", line_num + 1 ))?;

        tokens.extend(line_tokens);
    }
    Ok(tokens)
}
