mod array;
mod line;

use std::fmt::Display;

pub use array::Array;
pub use fetch_derive::Fetch;
pub use line::{Line, SEPARATOR};

pub trait Fetch: Display + Clone {
    fn name(&self) -> &'static str;

    fn as_fetchlines(&self) -> Vec<Line> {
        vec![self.clone().into()]
    }
}

/// Implemented by every top-level module. The default methods convert between
/// the concrete type and `serde_json::Value` so Machine can store all modules
/// in a single `IndexMap` without knowing their types at compile time.
///
/// `load_module` is the only method modules need to implement; the rest are
/// derived from it and the Serialize/Deserialize bounds.
pub trait DynModule: serde::Serialize + for<'de> serde::Deserialize<'de> + Fetch + Sized {
    fn load_module() -> Option<Self>;

    fn load_dyn() -> Option<serde_json::Value> {
        Self::load_module().and_then(|v| serde_json::to_value(&v).ok())
    }

    fn display_dyn(val: serde_json::Value) -> Option<Vec<Line>> {
        serde_json::from_value::<Self>(val)
            .ok()
            .map(|v| v.as_fetchlines())
    }

    // Returns None for every module except OsInfo, which overrides this via
    // the `colour = "field"` attribute on #[fetch(...)].
    fn colour_dyn(_val: &serde_json::Value) -> Option<String> {
        None
    }
}

/// Static record submitted to inventory by each module's #[derive(Fetch)].
/// Machine iterates these at runtime — adding a module requires no changes here.
pub struct ModuleRegistration {
    pub key: &'static str,
    pub priority: u32,
    pub load: fn() -> Option<serde_json::Value>,
    pub display: fn(serde_json::Value) -> Option<Vec<Line>>,
    pub colour: fn(&serde_json::Value) -> Option<String>,
}

// Must be called exactly once per type in the crate; pairing it with the
// struct definition keeps the two in sync.
inventory::collect!(ModuleRegistration);
