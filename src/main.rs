mod chunk;
mod value;
mod vm;

fn main() {
    let mut vm = vm::VM::new();

    let mut chunk = chunk::Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(chunk::OP_CONSTANT, 123);
    chunk.write(constant, 123);

    let constant = chunk.add_constant(3.4);
    chunk.write(chunk::OP_CONSTANT, 123);
    chunk.write(constant, 123);

    chunk.write(chunk::OP_ADD, 123);

    let constant = chunk.add_constant(5.6);
    chunk.write(chunk::OP_CONSTANT, 123);
    chunk.write(constant, 123);

    chunk.write(chunk::OP_DIVIDE, 123);

    chunk.write(chunk::OP_NEGATE, 123);

    chunk.write(chunk::OP_RETURN, 123);

    chunk.disassemble("test chunk");

    vm.interpret(chunk);
}
