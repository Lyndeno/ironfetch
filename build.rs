use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use std::env;
use std::io::Error;

#[path = "src/memunit.rs"]
mod memunit;

#[path = "src/args.rs"]
mod args;
use crate::args::Args;

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = <Args as CommandFactory>::command();
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "myapp", outdir.clone())?;
    }

    Ok(())
}
