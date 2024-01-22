use crate::lex::Symbol;
use crate::lex::Token;
use crate::grammar;

//Define abstract syntax tree
struct AST_Program {
    functions: Vec<AST_Function>,
}

struct AST_Function {
    name: String,
    return_type: String,
    statements: Vec<AST_Statement>,
}

struct AST_Statement {
    action: Statement_Action,
    expression: AstExpression,
}

type AstExpression = String;

enum Statement_Action {
    Return,
    If,
    While,
    For,
    Assign,
}

pub fn parse(tokens: Vec<(Symbol, Token)>) {// -> AST_Program {
    let g = grammar::get_grammar();

    for token in &tokens {
        match token.0 {
            Symbol::LBrace => println!("LBrace: {}", token.1),
            Symbol::RBrace => println!("RBrace: {}", token.1),
            Symbol::LParen => println!("LParen: {}", token.1),
            Symbol::RParen => println!("RParen: {}", token.1),
            Symbol::Semicolon => println!("Semicolon: {}", token.1),
            Symbol::Keyword => println!("Keyword: {}", token.1),
            Symbol::TypeKeyword => println!("Keyword: {}", token.1),
            Symbol::Identifier => println!("Identifier: {}", token.1),
            Symbol::IntLiteral => println!("IntLiteral: {}", token.1),
            Symbol::NonTerminal => ()
        }
    }

    // return AST_Program{
    //     // functions: Fns,
    // };
}

fn parse_statement(t: Token, s: String) {
    
}