use crate::compiler::Compiler;

mod compiler;

fn main() {
    let mut compiler = Compiler {};
    
    compiler.compile("println('Hello')println('World')println('Hello')");
}
