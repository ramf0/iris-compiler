#[derive(Debug)]
pub enum Expression<'a> {
    StringLiteral(&'a str),
}