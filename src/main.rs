use clap::{Parser, Subcommand};
use parm_assembler::{export_to_logisim, make_program, parse_lines, ExportError, LOGISIM_HEADER};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Subcommand, Debug)]
enum Command {
    /// Export a file to a logisim ROM
    Assemble {
        /// The input file or directory
        input: PathBuf,
    },
    /// Print a single instruction
    Print {
        /// The instruction
        instruction: String,
    },
    /// Interactive mode
    Repl,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn list_files(path: PathBuf) -> Vec<PathBuf> {
    if let Ok(entries) = fs::read_dir(&path) {
        entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .collect::<Vec<_>>()
    } else {
        vec![path]
    }
    .into_iter()
    .filter(|path| path.is_file() && path.extension().map(|ext| ext == "s").unwrap_or(false))
    .collect()
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

fn process_file(path: &Path) -> Result<(), ExportError> {
    let contents = read_file(&path);

    let output = export_to_logisim(&contents)?;

    write_file(&path.with_extension("rom.bin"), &output.rom);
    write_file(&path.with_extension("ram.bin"), &output.ram);

    Ok(())
}

fn assemble(input: PathBuf) {
    let (succeeded, failed): (Vec<_>, Vec<_>) = list_files(input)
        .into_iter()
        .map(|path| (process_file(path.as_ref()), path))
        .partition(|(result, _)| result.is_ok());

    for (result, path) in failed {
        println!(
            "Failed to process {}: {}",
            path.display(),
            result.unwrap_err()
        );
    }
    for (_, path) in succeeded {
        println!("Processed {}", path.display());
    }
}

fn print(instr: &str) {
    let parsed = match parse_lines(instr) {
        Ok(parsed) => parsed,
        Err(e) => {
            println!("Failed to parse: {}", e);
            return;
        }
    };

    println!("Parsed lines: {:?}", parsed);

    let program = match make_program(parsed.clone()) {
        Ok(program) => program,
        Err(e) => {
            println!("Failed to make program: {}", e);
            return;
        }
    };

    let rom = program.instrs;
    println!("Binary: {rom}");

    let logisim = match export_to_logisim(instr) {
        Ok(logisim) => logisim,
        Err(e) => {
            println!("Failed to export to logisim: {}", e);
            return;
        }
    };

    let logisim_rom = logisim.rom.replace(LOGISIM_HEADER, "");
    println!("Logisim ROM: {logisim_rom}");
}

fn repl() {
    println!("Welcome to the parm assembler REPL!");
    println!("Type an instruction to print it, or type 'exit' to quit.");
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            break;
        }
        print(input);
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Assemble { input } => assemble(input),
        Command::Print { instruction } => print(&instruction),
        Command::Repl => repl(),
    }
}
