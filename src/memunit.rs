use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, ValueEnum, Serialize, Deserialize)]
pub enum MemUnits {
    /// Mebibytes (MiB)
    #[value(name = "mib")]
    MiB,
    /// Gibibytes (GiB)
    #[value(name = "gib")]
    GiB,
}
