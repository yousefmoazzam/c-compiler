use std::{
    collections::VecDeque,
    env::args,
    path::{Path, PathBuf},
};

use c_compiler::{emit, lex, parse};

static ASM_FILE_EXTENSION: &str = "s";

fn main() {
    let args: Vec<String> = args().collect();

    let input_filepath = Path::new(&args[1]);
    let asm_file_stem = input_filepath
        .file_stem()
        .expect("Expected filename for input C source file");
    let mut output_filepath = PathBuf::new();
    output_filepath.push(asm_file_stem);
    output_filepath.set_extension(ASM_FILE_EXTENSION);

    let c_source_code =
        std::fs::read_to_string(input_filepath).expect("Unable to read C source code file");
    let tokens = lex::lex(&c_source_code);
    let mut token_queue = VecDeque::from(tokens);
    let c_ast = parse::c::parse_program_definition(&mut token_queue);
    let ir_ast = parse::ir::parse_program_definition(c_ast);
    let asm_ast = parse::asm::first_pass::parse_program_definition(ir_ast);
    emit::emit(&output_filepath, asm_ast).unwrap();
}
