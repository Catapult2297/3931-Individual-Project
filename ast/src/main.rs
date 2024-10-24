/*
use std::error::Error;
use std::fs;
use std::io::Read;

fn build_file() {
    let current_dir: std::path::PathBuf =
        std::env::current_dir().expect("Unable to get current directory");
    let rust_file: String = std::fs::read_to_string(current_dir.join("test_files").join("test.rs"))
        .expect("Unable to read rust file");
    let ast: syn::File = syn::parse_file(&rust_file).expect("Unable to create AST from rust file");

    let _ = std::fs::write("code_output", quote::quote!(#ast).to_string());
}

fn main() {
    build_file();
}
*/

use std::env;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::process;

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    let mut file = File::create("code_output.txt")?;
    let _ = args.next(); // executable name

    let filename = match (args.next(), args.next()) {
        (Some(filename), None) => filename,
        _ => {
            eprintln!("Usage: dump-syntax path/to/filename.rs");
            process::exit(1);
        }
    };

    let src = read_to_string(&filename).expect("unable to read file");
    let syntax = syn::parse_file(&src).expect("unable to parse file");

    // Debug impl is available if Syn is built with "extra-traits" feature.
    write!(file, "{:#?}", syntax)
}
