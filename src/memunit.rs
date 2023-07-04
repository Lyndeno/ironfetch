use clap::ValueEnum;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum MemUnits {
    /// Megabytes
    MB,
    /// Gigabytes
    GB,
}
