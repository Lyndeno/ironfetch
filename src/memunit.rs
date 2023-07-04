use clap::ValueEnum;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum MemUnits {
    /// Mebibytes (MiB)
    #[value(name = "mib")]
    MiB,
    /// Gibibytes (GiB)
    #[value(name = "gib")]
    GiB,
}
