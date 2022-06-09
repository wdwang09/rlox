mod chunk;
mod compiler;
mod scanner;
mod value;
mod vm;
mod rlox;

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

    vm.interpret_chunk(chunk);

    // ===
    println!("================");
    // ===

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        eprintln!("[Main] Usage: rlox [script]");
        std::process::exit(64);
    }
    let mut rlox = rlox::Rlox::new();
    if args.len() == 2 {
        let file_name = &args[1];
        let code = rlox.run_file(file_name);
        if code != 0 {
            eprintln!("[Main] Failed when running file {}", file_name);
            std::process::exit(code);
        }
    } else {
        let code = rlox.repl();
        if code != 0 {
            std::process::exit(code);
        }
    }
}
