use crate::compiler::ast::expressions::Expression;

#[derive(Debug)]
pub enum Statement<'a> {
    CallFunction {
        name: &'a str,
        args: Vec<Expression<'a>>,
    },
}