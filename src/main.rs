mod chunk;
mod value;

fn main() {
    let mut chunk = chunk::Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(chunk::OP_CONSTANT, 123);
    chunk.write(constant, 123);

    chunk.write(chunk::OP_RETURN, 123);
    chunk.disassemble("test chunk");
}
