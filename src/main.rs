use parm_assembler::{export_to_logisim, ExportError};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

struct Args {
    input: Vec<PathBuf>,
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let binary = &args[0];
    if args.len() < 2 {
        panic!("Usage:{}  <input file>", binary);
    }

    let files = args
        .into_iter()
        .skip(1)
        .map(PathBuf::from)
        .flat_map(|path| {
            if let Ok(entries) = fs::read_dir(&path) {
                entries
                    .filter_map(|entry| entry.ok())
                    .map(|entry| entry.path())
                    .collect::<Vec<_>>()
            } else {
                vec![path]
            }
        })
        .filter(|path| path.is_file() && path.extension().map(|ext| ext == "s").unwrap_or(false))
        .collect();

    Args { input: files }
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

fn main() {
    let args = parse_args();

    let (succeeded, failed): (Vec<_>, Vec<_>) = args
        .input
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
