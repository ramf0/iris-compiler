use colored::Colorize;

pub struct Exception{
}

impl Exception {
    pub fn new() -> Self {
        Exception { }
    }

    pub fn throw(&self, type_ex: &str, message: &str,file: &str , line: usize, column: usize)  {
        let format = 
            format!("┌─Exception──┄┄┄\n│ {} -- {}\n│ file: {}\n│ on line: {} and column: {}\n└─Exception──┄┄┄",
                    type_ex, message, file, line, column);
        panic!("{}", format.red());
    }
}
