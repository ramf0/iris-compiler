use crate::compiler::ast::statements::Statement;

pub mod expressions;
pub mod statements;

pub struct Program<'a> {
    pub program: Vec<Statement<'a>>,
}

impl<'a> Program<'a> {
    pub fn new(program: Vec<Statement<'a>>) -> Self {
        Program { program: program }
    }
}