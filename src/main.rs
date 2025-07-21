use crate::lexer::lex;
use crate::parser::parse;
use clap::Parser;
use generator::arm64::generate;

mod ast;
mod generator;
mod lexer;
mod optimizer;
mod parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "dumbc")]
#[command(about = "compiles C code")]
#[command(version, about)]
struct Args {
    #[arg(value_name = "FILE", help = "The file to compile")]
    input_file: String,

    #[arg(short, long, help = "target architecture", default_value_t = std::env::consts::ARCH.to_string())]
    arch: String,

    #[arg(short, long, help = "target platform", default_value_t = std::env::consts::OS.to_string())]
    platform: String,

    #[arg(long, help = "debug mode")]
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.arch != "aarch64" {
        eprintln!("Only arm64 is supported. Found: {}", args.arch);
        std::process::exit(1);
    }

    if args.platform != "linux" && args.platform != "macos" {
        eprintln!(
            "Unsupported platform: {} (expected 'linux' or 'macos')",
            args.platform
        );
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(&args.input_file)?;
    let tokens = lex(&input).expect("Lexer failed");
    if args.debug {
        println!("parsed tokens {:?}", tokens);
    }

    let program = parse(&tokens).expect("Parser failed");
    if args.debug {
        println!("program: {}", program);
    }

    let asm = generate(&program, &args.platform, args.debug)?;

    let asm_path = args.input_file.replace(".c", ".s");
    if args.debug {
        println!("writing to {}", asm_path);
    }

    std::fs::write(&asm_path, asm)?;
    Ok(())
}
