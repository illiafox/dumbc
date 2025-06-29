use crate::lexer::lex;
use crate::parser::parse;
use generator::arm64::generate;

mod ast;
mod generator;
mod lexer;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1); // skip program name
    let input_path = args
        .next()
        .expect("Usage: dumbc <file.c> [--arch arch] [--platform macos|linux]");

    let mut arch = std::env::consts::ARCH.to_string();
    let mut platform = std::env::consts::OS.to_string(); // default to host OS (e.g. "linux", "darwin")

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--arch" => {
                arch = args.next().expect("Expected architecture after --arch");
            }
            "--platform" => {
                platform = args.next().expect("Expected platform after --platform");
            }
            _ => {}
        }
    }

    if arch == "aarch64" {
        arch = "arm64".to_string();
    }

    if arch != "arm64" {
        eprintln!("Only arm64 is supported. Found: {}", arch);
        std::process::exit(1);
    }

    if platform != "linux" && platform != "macos" {
        eprintln!(
            "Unsupported platform: {} (expected 'linux' or 'macos')",
            platform
        );
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(&input_path)?;
    let tokens = lex(&input).expect("Lexer failed");
    println!("{:?}", tokens);

    let program = parse(&tokens).expect("Parser failed");

    println!("{}", program);
    let asm = generate(&program, &platform)?;

    let asm_path = input_path.replace(".c", ".s");
    std::fs::write(&asm_path, asm)?;
    Ok(())
}
