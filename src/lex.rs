use regex::Regex;

#[derive(Debug, Clone)]
pub enum Symbol {
    LBrace,
    RBrace,
    LParen,
    RParen,
    Semicolon,
    Keyword,
    Identifier,
    IntLiteral,
}

#[derive(Debug)]
struct Token {
    symbol: Symbol,
    rx: Regex,
}

pub fn lex(contents: String)-> Vec<(Symbol, String)> {
    let tokens = [
        Token{ symbol: Symbol::LBrace, rx: Regex::new(r"\{").unwrap() },
        Token{ symbol: Symbol::RBrace, rx: Regex::new(r"\}").unwrap() },
        Token{ symbol: Symbol::LParen, rx: Regex::new(r"\(").unwrap() },
        Token{ symbol: Symbol::RParen, rx: Regex::new(r"\)").unwrap() },
        Token{ symbol: Symbol::Semicolon, rx: Regex::new(r";").unwrap() },
        Token{ symbol: Symbol::Keyword, rx: Regex::new(r"return").unwrap() },
        Token{ symbol: Symbol::Identifier, rx: Regex::new(r"[a-zA-Z]\w*").unwrap() },
        Token{ symbol: Symbol::IntLiteral, rx: Regex::new(r"\d+").unwrap() },
    ];

    //Combine regexes into one
    let mut combined_regex = String::new();
    for token in &tokens {
        combined_regex.push_str(&format!("|{}", token.rx.as_str()));
    }
    let combined_regex = Regex::new(&combined_regex[1..]).unwrap();

    let mut discovered_tokens: Vec<(Symbol, String)> = vec![];

    for line in contents.lines() {
        for capture in combined_regex.captures_iter(line) {
            for token in &tokens {
                if token.rx.is_match(&capture[0]) {
                    let discovered = (token.symbol.clone(), capture[0].to_string());
                    println!("{:?}", &discovered);
                    discovered_tokens.push(discovered);
                    break;
                }
            }
        }
    }

    discovered_tokens
}