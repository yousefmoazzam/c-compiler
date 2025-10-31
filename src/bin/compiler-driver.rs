//! Binary crate to invoke the various steps needed to use the compiler to produce machine code,
//! namely:
//! - preprocessor (external)
//! - compiler (the main binary crate)
//! - assembler and linker (external)

use std::{env::args, io::Write, path::Path, process::Command};

fn main() {
    let args = args().collect::<Vec<_>>();

    let input_filepath = Path::new(&args[1]);
    let filestem = input_filepath
        .file_stem()
        .expect("Expected filename for input C source file")
        .to_str()
        .expect("Expected filestem to be vaid utf-8");
    let out_dirpath = input_filepath
        .parent()
        .expect("Expected input C source filename to not be the empty string, nor the root dir");

    let mut preprocessed_filename = filestem.to_string();
    preprocessed_filename.push_str(".i");
    let mut preprocessed_filepath = out_dirpath.to_path_buf();
    preprocessed_filepath.push(preprocessed_filename);
    println!("Preprocessed output: {:?}", preprocessed_filepath);
    let preprocessor_out = generate_preprocessor_command(input_filepath, &preprocessed_filepath)
        .output()
        .expect("Expected to be able to run preprocessor command");
    std::io::stderr()
        .write_all(&preprocessor_out.stderr)
        .expect("Expected to be able to write stderr of preprocessor command");

    let mut compiled_filename = filestem.to_string();
    compiled_filename.push_str(".s");
    let mut compiled_filepath = out_dirpath.to_path_buf();
    compiled_filepath.push(compiled_filename);
    println!("Compiled output: {:?}", compiled_filepath);
    generate_compiler_command(&preprocessed_filepath)
        .output()
        .expect("Expected to be able to run compiler command");

    let mut executable_filename = filestem.to_string();
    executable_filename.push_str(".out");
    let mut executable_filepath = out_dirpath.to_path_buf();
    executable_filepath.push(executable_filename);
    println!("Executable output: {:?}", executable_filepath);
    let assembler_linker_out =
        generate_assembler_linked_command(&compiled_filepath, &executable_filepath)
            .output()
            .expect("Expected to be able to run assembler+linker command");
    std::io::stderr()
        .write_all(&assembler_linker_out.stderr)
        .expect("Expected to be able to write stderr of assembler+linker command");
}

fn generate_preprocessor_command(input_filepath: &Path, output_filepath: &Path) -> Command {
    let mut command = Command::new("gcc");
    command.args([
        "-E",
        "-P",
        input_filepath
            .to_str()
            .expect("Expected C source code filepath to be valid utf-8"),
        "-o",
        output_filepath
            .to_str()
            .expect("Expected output preprocessed C source code filepath to be valid utf-8"),
    ]);
    command
}

fn generate_compiler_command(input_filepath: &Path) -> Command {
    let mut command = Command::new("cargo");
    command.args([
        "run",
        "--bin",
        "c_compiler",
        "--",
        input_filepath
            .to_str()
            .expect("Expected output compiled filepath to be valid utf-8"),
    ]);
    command
}

fn generate_assembler_linked_command(input_filepath: &Path, output_filepath: &Path) -> Command {
    let mut command = Command::new("gcc");
    command.args([
        input_filepath
            .to_str()
            .expect("Expected input compiled filepath to be valid utf-8"),
        "-o",
        output_filepath
            .to_str()
            .expect("Expected output executable filepath to be valid utf-8"),
    ]);
    command
}
