mod ast;
mod errors;
mod lexer;
mod parser;
mod resolver;
mod source;
mod source_map;
mod symbols;
mod token;
mod instructions;
mod ir_builder;
mod legalizer;
mod register_allocator;
mod emitter;

use lexer::Lexer;
use parser::Parser;

use std::env;

use crate::errors::ErrorReporter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide an input file name as argument.");
    }

    let input_file_name = args.get(1).unwrap();

    println!("Running Torch compiler v0.1 ..");
    println!();

    // Source

    let mut source_map = source_map::SourceMap::new();
    source_map.add_from_file(input_file_name).unwrap();

    println!("Loaded source file `{}`.", input_file_name);

    // Lexing

    let mut lexer = Lexer::new(&mut source_map);
    println!();
    println!("Tokenizing ..");

    let result = lexer.read_all();
    let tokens = match result {
        Err(err) => {
            ErrorReporter::print(&source_map, &err);
            return;
        }
        Ok(tkns) => tkns,
    };

    println!("Done tokenizing.");

    println!();
    println!("Extracted tokens:");

    for token in &tokens {
        println!(
            "{:?}: {:?} @ {:?} {:?}",
            token.token_type, token.value, token.position, token.source_id
        );
    }

    // Parsing

    let mut parser = Parser::new(tokens);

    println!();
    println!("Parsing syntax tree ..");

    let result = parser.parse_program();
    let statements = match result {
        Err(err) => {
            ErrorReporter::print(&source_map, &err);
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

    let mut symbol_table = symbols::SymbolTable::new();

    let resolver = resolver::Resolver::new(&mut symbol_table);
    println!();
    println!("Resolving ..");

    resolver.resolve_program(&statements).unwrap_or_else(|err| {
        ErrorReporter::print(&source_map, &err);
        return;
    });

    println!("Done resolving.");

    println!();
    println!("Symbols:");

    for (i, scope) in symbol_table.scopes.iter().enumerate() {
        println!(" Scope {}, parent {:?}:", i, scope.parent);

        for (_, symbol) in &scope.symbols {
            println!(
                "  {}: position {}, source_id {}",
                symbol.name, symbol.position, symbol.source_id,
            );
        }
    }

    // Intermediate Representation

    let mut ir_builder = ir_builder::IrBuilder::new(&mut symbol_table);
    println!();
    println!("Generating Intermediate Representation ..");

    let instrs = ir_builder.build(&statements);

    println!("Done generating IR.");

    println!();
    println!("Generated Instructions:");

    for instr in instrs {
        println!("{:?}", instr);
    }

    // Legalization

    let mut legalizer = legalizer::Legalizer::new(instrs);

    println!();
    println!("Legalizing Instructions ..");
    let legalized_instrs = legalizer.legalize();
    println!("Done legalizing.");

    println!();
    println!("Legalized Instructions:");
    for instr in &legalized_instrs {
        println!("{:?}", instr);
    }

    // Register Allocation

    let mut allocator = register_allocator::Allocator::new();
    println!();
    println!("Allocating Registers ..");
    let allocated_instrs = allocator.allocate(&legalized_instrs);
    println!("Done allocating registers.");

    println!();
    println!("Register Allocated Instructions:");
    for instr in allocated_instrs {
        println!("{:?}", instr);
    }
}
