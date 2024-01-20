use std::any::Any;

use crate::lex::Symbol;

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
    expression: AST_Expression,
}

struct AST_Expression {
    value: Any,
}

enum Statement_Action {
    Return,
    If,
    While,
    For,
    Assign,
}

#[derive(Debug)]
struct Grammar {
    program: Vec<String>,
    function: Vec<String>,
    statement: Vec<String>,
    expression: Vec<String>,
}

pub fn parse(tokens: Vec<(Symbol, String)>) -> AST_Program {
    let grammar = get_grammar();

    println!("Grammar: {:?}", grammar);

    for token in tokens {
        match token.0 {
            Symbol::Keyword => println!("Keyword: {}", token.1),
            Symbol::Identifier => println!("Identifier: {}", token.1),
            Symbol::IntLiteral => println!("IntLiteral: {}", token.1),
            _ => (),
        }
    }

    return AST_Program{
        functions: Fns,
    };
}

fn get_grammar() -> Grammar {
    let grammar_file = std::fs::read_to_string("config/grammar.bnf")
        .expect("There was an error reading the grammar file. Does config/grammar.bnf exist?");
    let grammar_str = grammar_file.to_string();

    let mut grammar = Grammar{
        program: vec![],
        function: vec![],
        statement: vec![],
        expression: vec![],
    };

    for line in grammar_str.lines() {
        let mut s = line.split("::=");

        let rule = s.next().expect("There was an error parsing the grammar file.");
        let rule = rule.trim();

        println!("Rule: {}", rule);

        let terminals = s.next().expect("There was an error parsing the grammar file.");
        let terminals: Vec<String> = terminals
            .split("|")
            .map(|s| s.trim().to_string())
            .collect();

        match rule {
            "<program>" => grammar.program = terminals.clone(),
            "<function>" => grammar.function = terminals.clone(),
            "<statement>" => grammar.statement = terminals.clone(),
            "<exp>" => grammar.expression = terminals.clone(),
            _ => (),
        };
    }

    grammar
}

fn parse_statement(s: String) {
    // tok = tokens.next()
    // if tok.type != "RETURN_KEYWORD":
    //     fail()
    // tok = tokens.next()
    // if tok.type != "INT"
    //     fail()
    // exp = parse_exp(tokens) //parse_exp will pop off more tokens
    // statement = Return(exp)

    // tok = tokens.next()
    // if tok.type != "SEMICOLON":
    //     fail()

    // return statement
}