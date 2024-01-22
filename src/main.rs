mod lex;
mod parse;
mod grammar;

fn main() {
    let args = std::env::args()
        .collect::<Vec<String>>();
    let filename = args.iter().nth(1)
        .expect("No filename provided");
    let outfile = args.iter().nth(2);
    let _outfile = match outfile {
        Some(s) => s,
        None => &filename,
    };

    let file = std::fs::read_to_string(filename)
        .expect("There was an error reading the file.");

    let tokens = lex::lex(file);
    parse::parse(tokens);
}
