use std::io::Write;
use crate::vm;
use crate::vm::InterpretResult;

pub struct Rlox {
    vm: vm::VM,
}

impl Rlox {
    pub fn new() -> Rlox {
        Rlox {
            vm: vm::VM::new(),
        }
    }

    pub fn run_file(&mut self, path: &String) -> i32 {
        let file = std::fs::read_to_string(path);
        return match file {
            Ok(data) => {
                let status = self.vm.interpret(data);
                match status {
                    InterpretResult::InterpretOk => 0,
                    InterpretResult::InterpretCompileError => 65,
                    InterpretResult::InterpretRuntimeError => 70,
                }
            }
            Err(err) => {
                eprintln!("[File] {}", err);
                74
            }
        };
    }

    pub fn repl(&mut self) -> i32 {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        loop {
            print!("> ");
            match stdout.flush() {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("[Prompt] {}", err);
                    return 1;
                }
            }
            let mut buffer = String::new();
            match stdin.read_line(&mut buffer) {
                Ok(_) => {
                    self.vm.interpret(buffer);
                }
                Err(err) => {
                    eprintln!("[Prompt] {}", err);
                    return 1;
                }
            };
        }
    }
}