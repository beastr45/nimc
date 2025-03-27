//! main entry point for the nimc compiler wrapper
//!

use std::path::Path;
use std::process;
use std::process::Command;
use std::{io, path::PathBuf};

use clap::Parser;
use log::{debug, error, info, trace, warn};

// nimc lib imports
use nimc::cli::{Cli, CompilerMode, init_logging};

/// The preprocess function will take in the location of a c source file and preprocess it
/// Writes a preprocessed file to disk.
fn preprocess(input_path: &Path) -> Result<PathBuf, io::Error> {
    debug!("Started preprocessing");
    let output_path = input_path.with_extension("i");

    let path_str = input_path.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Input file path is not valid UTF-8",
        )
    })?;
    let output_path_str = output_path.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "output file path is not valid UTF-8",
        )
    })?;

    debug!("Input: {}, Output: {}", path_str, output_path_str);

    match Command::new("gcc")
        .args(["-E", "-P", path_str, "-o", output_path_str])
        .status()
    {
        Ok(status) if status.success() => {
            trace!("Preprocessing step was successful");
            Ok(output_path)
        }
        Ok(status) => {
            error!("Preprocessing failed with status {}", status);
            Err(io::Error::new(io::ErrorKind::Other, "Preprocessing failed"))
        }
        Err(e) => {
            error!("Error executing gcc: {}", e);
            Err(e)
        }
    }
}

/// The assemble function will take in file location of assembly and then assemble the code.
/// Writes a binary to disk
/// If specified, the assembly code is deleted after
fn assemble(input_path: &Path) -> Result<PathBuf, io::Error> {
    debug!("Started Assembling");
    let output_path = input_path.with_extension("");

    let input_file_str = input_path.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Input file path is not valid UTF-8",
        )
    })?;
    let output_path_str = output_path.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "output file path is not valid UTF-8",
        )
    })?;

    debug!("Input: {}, Output: {}", input_file_str, output_path_str);

    // gcc ASSEMBLY_FILE -o output_path
    match Command::new("gcc")
        .args([input_file_str, "-o", output_path_str])
        .status()
    {
        Ok(status) if status.success() => {
            trace!("Assembler step was successful");
            Ok(output_path)
        }
        Ok(status) => {
            error!("Assembler failed with status {}", status);
            Err(io::Error::new(io::ErrorKind::Other, "Assembler failed"))
        }
        Err(e) => {
            error!("Error while executing gcc: {}", e);
            Err(e)
        }
    }
}

fn lex() {
    warn!("Lexing has not been implemented");
}
fn parse() {
    warn!("parsing has not been implemented");
}
fn gen_asm() {
    warn!("parsing has not been implemented");
}
fn code_emission() {
    warn!("code_emission has not been implemented");
}
fn gcc_compile(input_path: &Path) -> Result<PathBuf, io::Error> {
    // The Command to emit clean assembly is the following:
    // gcc -S -O -fno-asynchronous-unwind-tables -fcf-protection=none INPUT FILE

    debug!("Compiling w/ gcc...");
    let output_path = input_path.with_extension("s");

    let input_path_str = input_path.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Input file path is not valid UTF-8",
        )
    })?;
    let output_path_str = output_path.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Output file path is not valid UTF-8",
        )
    })?;

    debug!("Input: {}, Output: {}", input_path_str, output_path_str);

    match Command::new("gcc")
        .args([
            "-S",
            "-O",
            "-fno-asynchronous-unwind-tables",
            "-fcf-protection=none",
            input_path_str,
            "-o",
            output_path_str,
        ])
        .status()
    {
        Ok(status) if status.success() => {
            trace!("Gcc compile step was successful");
            Ok(output_path)
        }
        Ok(status) => {
            error!("Gcc compile failed with status {}", status);
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Compiling w/ gcc failed",
            ))
        }
        Err(e) => {
            error!("Error while executing gcc: {}", e);
            Err(e)
        }
    }
}

/// Compile is a function that will do all of the heavy lifting.
/// Compile takes in file location argument for the preprocessed file and then
/// writes out an assembled file to disk.
/// If specified, the preprocessed file is deleted after
/// #TODO: overhaul the error return types for the compile method
fn compile(input_path: &Path) -> Result<PathBuf, io::Error> {
    info!("Beginning the compilation process");
    // we will compile with gcc to test the wrapper
    let output_path = gcc_compile(input_path)?;

    lex();
    parse();
    gen_asm();
    code_emission();

    Ok(output_path)
}
fn run_compiler(input_path: &Path, mode: CompilerMode) -> Result<(), i32> {
    //TODO: Implement pipeline pattern?
    match mode {
        CompilerMode::Full => {
            let preprocessed = preprocess(input_path).map_err(|e| {
                error!("Preprocessing failed: {}", e);
                1
            })?;

            let compiled = compile(&preprocessed).map_err(|e| {
                error!("Compilation failed: {}", e);
                1
            })?;

            assemble(&compiled).map_err(|e| {
                error!("Assembly failed: {}", e);
                1
            })?;

            info!("Compilation completed successfully");
            Ok(())
        }
        CompilerMode::EmitAsm => {
            info!("Assembly mode");
            error!("This mode has not yet been implemented");
            process::exit(1);
            // preprocess();
            // compile();
        }
        _ => {
            error!("this mode has not yet been implemented");
            process::exit(1);
        }
    }
}

fn main() {
    let cli = Cli::parse();
    init_logging(cli.log_level());

    // First make sure that all arguments are correct
    let result = match cli.validate_input() {
        Ok(input_path) => run_compiler(&input_path, cli.mode()),
        Err(e) => {
            error!("Input validation failed: {}", e);
            Err(1)
        }
    };

    //Return sucess if no errs occur
    process::exit(match result {
        Ok(_) => 0,
        Err(e) => e,
    });
}
