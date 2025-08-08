use crate::compiler::syntax::Parser;

mod ast;
mod syntax;
pub mod exception;

pub struct Compiler {
    
}

impl Compiler {

    pub fn compile(&mut self, input: &str) -> String {
        let parser = Parser::new(input);
        
        for statement in parser {
            println!("{:?}", statement);
        }
        
        "---".to_string()
    }
}