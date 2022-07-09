mod declarations;
mod function;
mod functions;
mod ty;
mod value;

pub use declarations::*;
pub use function::*;
pub use functions::*;
pub use ty::*;
pub use value::*;

use crate::{ast, error::Error, ir::Blocks};

pub fn compile_program(program: ast::Program) -> Result<(), Error> {
    let mut types = Types::default();
    let mut signatures = FunctionSignatures::default();
    let declarations = Declarations::from_program(program)?;
    let functions = Functions::new(&declarations, &mut types, &mut signatures)?;
    let function_compiler = FunctionCompiler::new(&declarations, &functions);

    let mut blocks = Blocks::new();
    function_compiler.compile_program(&mut blocks, &mut types, &mut signatures)?;

    println!("{:#?}", blocks);

    Ok(())
}
