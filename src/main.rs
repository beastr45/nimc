//! main entry point for the nimc compiler wrapper
//!

use core::error;
use std::process;

use clap::Parser;
use log::{debug, error, info, trace, warn};

// nimc lib imports
use nimc::cli::{Cli, init_logging};
use nimc::driver::run_compiler;

fn main() {
    let cli = Cli::parse();
    init_logging(cli.log_level());

    // First make sure that all arguments are correct
    let result = match cli.validate_input() {
        Ok(input_path) => run_compiler(&input_path, cli.mode()).map_err(|e| {
            error!("{}",e);
            for cause in e.chain().skip(1) {
                error!("Caused by: {}", cause);
            }
            // error!("{e}");
            1
        }),
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
