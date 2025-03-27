use clap::{ArgGroup, Parser};
use std::path::PathBuf;

// here we will write the required code to add logging levels
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

/// This function will set up logging based on the users prefrence
pub fn init_logging(level:LevelFilter) {
    TermLogger::init(
        level,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .unwrap();
}

#[derive(Debug)]
pub enum CompilerMode {
    Lex,
    Parse,
    Codegen,
    EmitAsm,
    Full,
}

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about = "Nimble C Hobby Compiler",
    long_about = None,
    name = "nimc",
    arg_required_else_help = true,
    group(
        ArgGroup::new("mode")
            .args(["lex", "parse", "codegen", "emit_asm"])
            .multiple(false)
    ),
    group(
        ArgGroup::new("loglevel")
            .args(["verbose","quiet"])
            .multiple(false)
    ),
)]
pub struct Cli {
    /// Path to the source code file
    pub file_path: PathBuf,
    /// Perform lexical analysis and output tokens
    #[arg(long)]
    pub lex: bool,

    /// Perform parsing and output AST
    #[arg(long)]
    pub parse: bool,

    /// Perform code generation
    #[arg(long)]
    pub codegen: bool,

    /// Emit assembly file instead of executable
    #[arg(short = 'S')]
    pub emit_asm: bool,

    /// Set log level to verbose (debug info)
    #[clap(long, short)]
    pub verbose: bool,

    /// Set log level to quiet (hide warnings)
    #[clap(long, short)]
    pub quiet: bool,

    //TODO:
    /// Set log level to trace (overkill messages)
    #[clap(long)]
    pub trace: bool,
}

impl Cli {

    /// Return the currrent compilation mode
    pub fn mode(&self) -> CompilerMode {
        if self.lex {
            CompilerMode::Lex
        } else if self.parse {
            CompilerMode::Parse
        } else if self.codegen {
            CompilerMode::Codegen
        } else if self.emit_asm {
            CompilerMode::EmitAsm
        } else {
            CompilerMode::Full
        }
    }

    /// validate_input will check to make sure that the file path provided is
    /// correct and then returns the file path
    pub fn validate_input(&self) -> Result<PathBuf, String> {
        if self.file_path.extension().map(|e| e != "c").unwrap_or(true) {
            return Err("Input file must have .c extension".to_string());
        }
        if !self.file_path.exists() {
            return Err("Input file does not exist".to_string());
        }
        if !self.file_path.is_file() {
            return Err("Path is not a file".to_string());
        }
        Ok(self.file_path.to_path_buf())
    }

    /// return the desired output path
    pub fn output_path(&self, mode: &CompilerMode) -> PathBuf {
        self.file_path.with_extension(match mode {
            CompilerMode::EmitAsm => "s",
            _ => "",
        })
    }

    /// Get a struct representing log mode set by user
    /// Pair this getter with the init_logging function
    pub fn log_level(&self) -> LevelFilter{
        if self.verbose {
            LevelFilter::Debug
        } else if self.quiet {
            LevelFilter::Error
        }else {
            LevelFilter::Info
        }

    }
}
