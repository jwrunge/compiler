use crate::lex;

#[derive(Debug)]
pub struct Grammar {
    program: Vec<Vec<(lex::Symbol, lex::Token)>>,
    function: Vec<Vec<(lex::Symbol, lex::Token)>>,
    statement: Vec<Vec<(lex::Symbol, lex::Token)>>,
    expression: Vec<Vec<(lex::Symbol, lex::Token)>>,
}

pub fn get_grammar() -> Grammar {
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

        let terminals = s.next().expect("There was an error parsing the grammar file.");
        let terminals: Vec<String> = terminals
            .split("|")
            .map(|s| s.trim().to_string())
            .collect();

        println!("{:?}", terminals);

        match rule {
            "<program>" => grammar.program = lex_grammar(terminals),
            "<function>" => grammar.function = lex_grammar(terminals),
            "<statement>" => grammar.statement = lex_grammar(terminals),
            "<exp>" => grammar.expression = lex_grammar(terminals),
            _ => (),
        };
    }

    println!("{:#?}", grammar);
    grammar
}

fn lex_grammar(terminals: Vec<String>) -> Vec<Vec<(lex::Symbol, lex::Token)>> {
    let mut ts: Vec<Vec<(lex::Symbol, lex::Token)>> = vec![];
    for terminal in terminals {
        let mut components: Vec<(lex::Symbol, lex::Token)> = vec![];

        for w in terminal.split_whitespace() {
            println!("{}", w);
            //Handle literal match
            if w.starts_with("\"") && w.ends_with("\"") {
                let literal = w.trim_matches('"');
                for token in lex::TOKENS.iter() {
                    if token.rx.is_match(&literal) {
                        components.push((token.symbol.clone(), literal.to_string()));
                    }
                }
                continue;
            }

            //Handle non-terminal match
            if w.starts_with("<") && w.ends_with(">") {
                let non_terminal = w.trim_matches(|c| c == '<' || c == '>');
                match non_terminal {
                    "program" => components.push((lex::Symbol::NonTerminal, "<program>".to_string())),
                    "function" => components.push((lex::Symbol::NonTerminal, "<function>".to_string())),
                    "statement" => components.push((lex::Symbol::NonTerminal, "<statement>".to_string())),
                    "exp" => components.push((lex::Symbol::NonTerminal, "<exp>".to_string())),
                    _ => (),
                }
                continue;
            }
        }

        ts.push(components);
    }

    ts
}

pub fn match_function() {
    
}

pub fn match_statement() {
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

pub fn match_expression() {

}