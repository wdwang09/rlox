use crate::value;

pub const OP_RETURN: u8 = 1;
pub const OP_CONSTANT: u8 = 2;

pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<u32>,
    constants: value::ValueArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: vec![],
            lines: vec![],
            constants: value::ValueArray::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.code.len()
    }

    pub fn write(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: value::Value) -> u8 {
        self.constants.write(value);
        (self.constants.count() - 1).try_into().expect("Index overflow!")
    }

    // ===

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.count() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:04} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        match instruction {
            OP_CONSTANT => self.constant_instruction("OP_CONSTANT", offset),
            OP_RETURN => self.simple_instruction("OP_RETURN", offset),
            _ => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            }
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        print!("{:-16} {:4} '", name, constant);
        self.constants.print_value(constant as usize);
        println!("'");
        offset + 2
    }
}
