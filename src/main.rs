//! main entry point for the nimc compiler wrapper

use clap::{Parser, Subcommand};

use std::io;
use std::path::Path;
use std::process::{Command, ExitStatus, exit};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Subcommand)]
enum Commands {
    Test {
        #[arg(short, long)]
        list: bool,
    },
}

/// The preprocess function will take in the location of a c source file and preprocess it
/// Writes a preprocessed file to disk.
fn preprocess() -> Result<(), io::Error> {
    println!("Preprocessing...");
    let input_file = Path::new("main.c");
    let output_file = input_file.with_extension("i");

    let input_file_str = input_file.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Input file path is not valid UTF-8",
        )
    })?;
    let output_file_str = output_file.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "output file path is not valid UTF-8",
        )
    })?;

    // Check the result
    println!("Preprocessing Input file: {:?}", input_file);
    println!("Preprocessing Output file: {:?}", output_file);

    match Command::new("gcc")
        .args(["-E", "-P", input_file_str, "-o", output_file_str])
        .status()
    {
        Ok(status) if status.success() => {
            println!("Preprocessing successful");
            Ok(())
        }
        Ok(status) => {
            eprintln!("Preprocessing failed with status {}", status);
            Err(io::Error::new(io::ErrorKind::Other, "Preprocessing failed"))
        }
        Err(e) => {
            eprintln!("Error while executing gcc: {}", e);
            Err(e)
        }
    }
}

/// The assemble function will take in file location of assembly and then assemble the code.
/// Writes a binary to disk
/// If specified, the assembly code is deleted after
fn assemble() -> Result<(), io::Error> {
    println!("Assembling...");
    let input_file = Path::new("main.s");
    let output_file = input_file.with_extension("");

    let input_file_str = input_file.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Input file path is not valid UTF-8",
        )
    })?;
    let output_file_str = output_file.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "output file path is not valid UTF-8",
        )
    })?;

    // Check the result
    println!("Assembler Input file: {:?}", input_file);
    println!("Assemble Output file: {:?}", output_file);


// gcc ASSEMBLY_FILE -o OUTPUT_FILE
    match Command::new("gcc")
        .args([input_file_str, "-o", output_file_str])
        .status()
    {
        Ok(status) if status.success() => {
            println!("Assembler successful");
            Ok(())
        }
        Ok(status) => {
            eprintln!("Assembler failed with status {}", status);
            Err(io::Error::new(io::ErrorKind::Other, "Assembler failed"))
        }
        Err(e) => {
            eprintln!("Error while executing gcc: {}", e);
            Err(e)
        }
    }
}

fn lex() {
    println!("Lexing has not been implemented");
}
fn parse() {
    println!("parsing has not been implemented");
}
fn gen_asm() {
    println!("parsing has not been implemented");
}
fn code_emission() {
    println!("code_emission has not been implemented");
}
fn gcc_compile() -> Result<(), io::Error> {
    // The Command to emit clean assembly is the following:
    // gcc -S -O -fno-asynchronous-unwind-tables -fcf-protection=none INPUT FILE

    println!("Comipiling w/ gcc...");
    let input_file = Path::new("main.i");
    let output_file = input_file.with_extension("s");

    let input_file_str = input_file.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Input file path is not valid UTF-8",
        )
    })?;
    let output_file_str = output_file.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Output file path is not valid UTF-8",
        )
    })?;

    // Check the result
    println!("Compilation Input file: {:?}", input_file);
    println!("Compilation Output file: {:?}", output_file);

    match Command::new("gcc")
        .args([
            "-S",
            "-O",
            "-fno-asynchronous-unwind-tables",
            "-fcf-protection=none",
            input_file_str,
            "-o",
            output_file_str,
        ])
        .status()
    {
        Ok(status) if status.success() => {
            println!("Gcc compile successful");
            Ok(())
        }
        Ok(status) => {
            eprintln!("Gcc compile failed with status {}", status);
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Compiling w/ gcc failed",
            ))
        }
        Err(e) => {
            eprintln!("Error while executing gcc: {}", e);
            Err(e)
        }
    }
}

/// Compile is a function that will do all of the heavy lifting.
/// Compile takes in file location argument for the preprocessed file and then
/// writes out an assembled file to disk.
/// If specified, the preprocessed file is deleted after
/// #TODO: overhaul the error return types for the compile method
fn compile() -> Result<(), io::Error> {
    println!("Compiling...");
    // we will compile with gcc to test the wrapper
    gcc_compile()?;

    lex();
    parse();
    gen_asm();
    code_emission();

    Ok(())
}

fn main() {
    // let args = Args::parse();
    // println!("{:?}", args);

    // TODO: make this into a match statment instead with file path as ok value
    if let Err(e) = preprocess() {
        println!("Fatal error {e}, exiting now");
        exit(1);
    }

    if let Err(e) = compile() {
        println!("Fatal error {e}, exiting now");
        exit(1);
    }

    if let Err(e) = assemble() {
        println!("Fatal error {e}, exiting now");
        exit(1);
    }
}
