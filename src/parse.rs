use crate::lex::Symbol;

//Define abstract syntax tree
struct AST {

}

#[derive(Debug)]
struct Grammar {
    program: Vec<String>,
    function: Vec<String>,
    statement: Vec<String>,
    expression: Vec<String>,
}

pub fn parse(tokens: Vec<(Symbol, String)>) {
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
            .trim()
            .split("|")
            .map(|s| s.to_string())
            .collect();

        match rule {
            "<program>" => grammar.program = terminals.clone(),
            "<function>" => grammar.function = terminals.clone(),
            "<statement>" => grammar.statement = terminals.clone(),
            "<expression>" => grammar.expression = terminals.clone(),
            _ => (),
        };
    }

    grammar
}