use clap::ValueEnum;

#[derive(Copy, Clone, Debug)]
pub enum MemUnits {
    MB,
    GB,
}

impl ValueEnum for MemUnits {
    fn value_variants<'a>() -> &'a [Self] {
        &[MemUnits::GB, MemUnits::MB]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            MemUnits::GB => clap::builder::PossibleValue::new("gb").help("Gigabytes"),
            MemUnits::MB => clap::builder::PossibleValue::new("mb").help("Megabytes"),
        })
    }
}
