mod lexer;
mod token;
mod parser;
mod ast;
mod source;
mod errors;
mod resolver;
mod symbols;

use lexer::Lexer;
use parser::Parser;

use std::{env, fs};

use crate::{errors::ErrorReporter, source::Source};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide an input file name as argument.");
    }

    let input_file_name = args.get(1).unwrap();

    println!("Running Torch compiler v0.1 ..");
    println!();

    // Source

    println!("Reading code file '{}' ..", input_file_name);
    let file_content = fs::read_to_string(input_file_name).expect("Could not read file");
    println!("Done reading code file with {} characters.", file_content.len());

    println!();
    println!("Read code:\n{}", file_content);

    // Lexing

    let source = Source::new(file_content, input_file_name.clone());
    let mut lexer = Lexer::new(&source);

    println!();
    println!("Tokenizing ..");

    let result = lexer.read_all();
    let tokens = match result {
        Err(err) => {
            ErrorReporter::print(&source, &err);
            return;
        },
        Ok(tkns) => tkns,
    };

    println!("Done tokenizing.");

    println!();
    println!("Extracted tokens:");

    for token in &tokens {
        println!("{:?}: {:?}", token.token_type, token.value);
    }

    // Parsing

    let mut parser = Parser::new(tokens);

    println!();
    println!("Parsing syntax tree ..");

    let result = parser.parse_program();
    let statements = match result {
        Err(err) => {
            ErrorReporter::print(&source, &err);
            return;
        }
        Ok(stmts) => stmts,
    };

    println!("Done parsing.");

    println!();
    println!("Parsed statements:");
    for stmt in &statements {
        println!("{:?}", stmt)
    }

    // Resolving

    let resolver = resolver::Resolver::new();
    println!();
    println!("Resolving ..");

    let result = resolver.resolve_program(&statements);
    match result {
        Err(errors) => {
            for err in &errors {
                ErrorReporter::print(&source, err);
            }
            return;
        }
        Ok(()) => {
            println!("Done resolving.");
        }
    }

    // Intermediate Representation

    // Code Generation
}