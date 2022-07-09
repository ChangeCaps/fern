use std::{fs::read_to_string, path::PathBuf};

use fern::{ast, compiler::compile_program, parse::Parser, source::SourceId};

#[derive(clap::Parser)]
#[clap(author, version)]
struct Args {
    path: PathBuf,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    let source = read_to_string(&args.path).unwrap();

    let mut parser = Parser::new(&source, SourceId::default());

    let program = parser.parse::<ast::Program>().unwrap();

    compile_program(program).unwrap();
}
