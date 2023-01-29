use parm_assembler::export_to_logisim;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

struct Args {
    input: PathBuf,
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let binary = &args[0];
    if args.len() != 2 {
        panic!("Usage:{}  <input file>", binary);
    }

    Args {
        input: PathBuf::from(args[1].clone()),
    }
}

fn read_file(path: &Path) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn write_file(path: &Path, contents: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

fn main() {
    let args = parse_args();
    let contents = read_file(&args.input);

    println!("Input file: {}", contents);

    let output = export_to_logisim(&contents).expect("Failed to export to logisim");

    println!("Output file: {}", output);

    let output_path = args.input.with_extension("bin");

    write_file(&output_path, &output);
}
