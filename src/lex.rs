use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub enum Symbol {
    LBrace,
    RBrace,
    LParen,
    RParen,
    Semicolon,
    Keyword,
    TypeKeyword,
    Identifier,
    IntLiteral,
    NonTerminal
}

#[derive(Debug)]
pub struct TokenDef {
    pub symbol: Symbol,
    pub rx: Regex,
}

pub type Token = String;

lazy_static! {
    pub static ref TOKENS: [TokenDef;9] = [
        TokenDef{ symbol: Symbol::LBrace, rx: Regex::new(r"\{").unwrap() },
        TokenDef{ symbol: Symbol::RBrace, rx: Regex::new(r"\}").unwrap() },
        TokenDef{ symbol: Symbol::LParen, rx: Regex::new(r"\(").unwrap() },
        TokenDef{ symbol: Symbol::RParen, rx: Regex::new(r"\)").unwrap() },
        TokenDef{ symbol: Symbol::Semicolon, rx: Regex::new(r";").unwrap() },
        TokenDef{ symbol: Symbol::Keyword, rx: Regex::new(r"return").unwrap() },
        TokenDef{ symbol: Symbol::TypeKeyword, rx: Regex::new(r"int").unwrap() },
        TokenDef{ symbol: Symbol::Identifier, rx: Regex::new(r"[a-zA-Z]\w*").unwrap() },
        TokenDef{ symbol: Symbol::IntLiteral, rx: Regex::new(r"\d+").unwrap() },
    ];
}

pub fn lex(contents: String)-> Vec<(Symbol, Token)> {
    //Combine regexes into one
    let mut combined_regex = String::new();
    for token in TOKENS.iter() {
        combined_regex.push_str(&format!("|{}", token.rx.as_str()));
    }
    let combined_regex = Regex::new(&combined_regex[1..]).unwrap();

    let mut discovered_tokens: Vec<(Symbol, Token)> = vec![];

    for line in contents.lines() {
        for capture in combined_regex.captures_iter(line) {
            for token in TOKENS.iter() {
                if token.rx.is_match(&capture[0]) {
                    let discovered = (token.symbol.clone(), capture[0].to_string());
                    discovered_tokens.push(discovered);
                    break;
                }
            }
        }
    }

    discovered_tokens
}