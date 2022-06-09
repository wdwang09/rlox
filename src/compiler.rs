use crate::scanner;

pub struct Compiler {
    scanner: scanner::Scanner,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            scanner: scanner::Scanner::new(),
        }
    }

    pub fn compile(&mut self, source: String) {
        self.scanner.scan(source);
    }
}
