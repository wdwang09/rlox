use crate::chunk;
use crate::value;

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    // InterpretRuntimeError,
}

const STACK_SIZE: usize = 256;

pub struct VM {
    chunk: chunk::Chunk,
    ip: usize,
    stack: [value::Value; STACK_SIZE],
    stack_idx: usize,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: chunk::Chunk::new(),
            ip: 0,
            stack: [0 as value::Value; STACK_SIZE],
            stack_idx: 0,
        }
    }

    pub fn interpret(&mut self, chunk: chunk::Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        return self.run();
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            if cfg!(debug_assertions) {
                print!("          ");
                for i in 0..self.stack_idx {
                    print!("[");
                    value::print_value(self.stack[i]);
                    print!("]");
                }
                println!();
                self.chunk.disassemble_instruction(self.ip);
            }
            let instruction = self.chunk.read_byte(self.ip);
            self.ip += 1;
            match instruction {
                chunk::OP_CONSTANT => {
                    let constant = self.chunk.read_constant(self.chunk.read_byte(self.ip) as usize);
                    self.ip += 1;
                    self.push(constant);
                }
                chunk::OP_NEGATE => {
                    let v = -self.pop();
                    self.push(v);
                }
                chunk::OP_RETURN => {
                    value::print_value(self.pop());
                    println!();
                    return InterpretResult::InterpretOk;
                }
                chunk::OP_ADD | chunk::OP_SUBTRACT | chunk::OP_MULTIPLY | chunk::OP_DIVIDE => {
                    self.binary_op(instruction);
                }
                _ => {
                    return InterpretResult::InterpretCompileError;
                }
            };
        }
    }

    fn binary_op(&mut self, instruction: u8) {
        let b = self.pop();
        let a = self.pop();
        match instruction {
            chunk::OP_ADD => {
                self.push(a + b);
            }
            chunk::OP_SUBTRACT => {
                self.push(a - b);
            }
            chunk::OP_MULTIPLY => {
                self.push(a * b);
            }
            chunk::OP_DIVIDE => {
                self.push(a / b);
            }
            _ => {
                panic!("The operator is invalid.");
            }
        };
    }

    fn push(&mut self, value: value::Value) {
        self.stack[self.stack_idx] = value;
        self.stack_idx += 1;
    }

    fn pop(&mut self) -> value::Value {
        self.stack_idx -= 1;
        self.stack[self.stack_idx]
    }
}
