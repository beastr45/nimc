use std::fs;
use std::path::Path;
use std::process::Command;
use std::{io, path::PathBuf};

use anyhow::{Context, Result};
use log::{debug, error, info, trace, warn};

use crate::cli::CompilerMode;
use crate::errors::CompilerError;
use crate::lexer::lex;

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
pub fn gcc_compile(input_path: &Path) -> Result<PathBuf, io::Error> {
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
//TODO: these will be seperate modules later
pub fn parse() -> Result<()> {
    unimplemented!();
}
pub fn gen_asm() -> Result<()> {
    unimplemented!();
}
pub fn code_emission() -> Result<()> {
    unimplemented!();
}

/// Compile is a function that will do all of the heavy lifting.
/// Compile takes in file location argument for the preprocessed file and then
/// writes out an assembled file to disk.
/// If specified, the preprocessed file is deleted after
fn compile(input_path: &Path, mode: &CompilerMode) -> Result<PathBuf> {

    info!("Beginning the compilation process");
    // we will compile with gcc to test the wrapper

    let output_path = input_path.with_extension("s");

    let tokens = lex(input_path).context("Lexing stage failed")?;

    if *mode == CompilerMode::Lex {
        info!("Stopped after lexing. Token Contents: {:?}", tokens);
        return Ok(output_path);
    }
    parse().context("Parsing stage failed")?;
    if *mode == CompilerMode::Parse {
        info!("Stopped after parsing");
        return Ok(output_path);
    }
    gen_asm().context("Assembly generation stage failed")?;
    if *mode == CompilerMode::Codegen {
        info!("Stopped after codegen");
        return Ok(output_path);
    }
    code_emission().context("Code emission stage failed")?;

    Ok(output_path)
}

/// Main function to run the compilation commands based on selected copiler mode
pub fn run_compiler(input_path: &Path, mode: CompilerMode) -> Result<()> {
    //TODO: implement a preprocessing compilation mode

    // let compiled_gcc = gcc_compile(input_path)?;
    // Stage 1: Preprocessing
    let preprocessed = preprocess(input_path).with_context(|| "Preprocessing step failed")?;

    // Stage 2: Compile
    // TODO: verify error pipeline to make sure error print and if multiple print in order. Printing errors shall be right after the compile function
    let compiled = compile(&preprocessed, &mode).with_context(|| "Compilation step failed")?;

    info!("Compilation completed successfully");

    // after compiling we clean up files from previous pipeline step
    fs::remove_file(preprocessed).with_context(|| "Cleaning up file of type .i failed")?;

    if mode == CompilerMode::EmitAsm || mode != CompilerMode::Full {
        info!("Skipping assembly step");
        return Ok(());
    }

    // Stage 3: Assemble
    assemble(&compiled).with_context(|| "Assembly step failed")?;

    // after assembling we clean up files from previous pipeline step
    fs::remove_file(compiled).with_context(|| "Cleaning up file of type .s failed: {}")?;

    Ok(())
}
