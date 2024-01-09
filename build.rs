use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use std::env;
use std::io::Error;
use std::path::PathBuf;

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

    let out_path = PathBuf::from(outdir.clone());

    let mut cmd = <Args as CommandFactory>::command();
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "ironfetch", outdir.clone())?;
    }

    let man = clap_mangen::Man::new(cmd);
    let mut man_buf = Vec::<u8>::default();
    man.render(&mut man_buf)?;

    std::fs::write(out_path.join("ironfetch.1"), man_buf)?;

    Ok(())
}
