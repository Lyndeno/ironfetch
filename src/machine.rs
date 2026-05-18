use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::colourblocks::colourblocks;
use crate::fetch::{Array, ModuleRegistration, SEPARATOR};
use crate::Result;

/// Controls which modules are loaded. Baseline is all modules unless `none` is
/// set, then it is empty. `show` adds to the baseline; `hide` removes from it.
/// Comparisons are case-insensitive so `--show gpu` matches the key `"GPU"`.
/// When both `show` and `hide` name the same module, `hide` wins.
#[derive(Default)]
pub struct ModuleFilter {
    pub none: bool,
    pub show: Vec<String>,
    pub hide: Vec<String>,
}

impl ModuleFilter {
    fn is_active(&self, key: &str) -> bool {
        let lower = key.to_lowercase();
        let in_show = self.show.iter().any(|s| s.to_lowercase() == lower);
        let in_hide = self.hide.iter().any(|s| s.to_lowercase() == lower);
        // active = (baseline ∪ show) \ hide
        (!self.none || in_show) && !in_hide
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Machine {
    // flatten collapses the map's key-value pairs into the top-level JSON
    // object, giving {"OS": {...}, "CPU": {...}} rather than {"modules": {...}}.
    #[serde(flatten)]
    modules: IndexMap<String, serde_json::Value>,
    // Not persisted; defaults to true so --input path still shows colour blocks.
    #[serde(skip, default = "Machine::default_colour_blocks")]
    pub colour_blocks: bool,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            modules: IndexMap::default(),
            colour_blocks: true,
        }
    }
}

impl Machine {
    fn default_colour_blocks() -> bool {
        true
    }

    pub fn new(filter: &ModuleFilter) -> Self {
        // inventory::iter order is not guaranteed, so sort by priority to
        // preserve the intended display order across builds.
        let mut entries: Vec<&ModuleRegistration> =
            inventory::iter::<ModuleRegistration>().collect();
        entries.sort_by_key(|e| e.priority);

        let known: Vec<&str> = entries.iter().map(|e| e.key).collect();
        for name in filter.show.iter().chain(filter.hide.iter()) {
            let lower = name.to_lowercase();
            if !known.iter().any(|k| k.to_lowercase() == lower) {
                eprintln!(
                    "warning: unknown module '{name}' (known: {})",
                    known.join(", ")
                );
            }
        }

        Self {
            modules: entries
                .iter()
                .filter(|e| filter.is_active(e.key))
                .filter_map(|e| Some((e.key.to_string(), (e.load)()?)))
                .collect(),
            colour_blocks: true,
        }
    }

    /// Read a machine from a json file
    ///
    /// # Errors
    /// Returns an error if reading fails
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let f = File::open(path)?;
        let r = BufReader::new(f);
        Ok(serde_json::from_reader(r)?)
    }

    /// Writes machine to a json file
    ///
    /// # Errors
    /// Returns an error if writing fails
    pub fn to_file(&self, path: PathBuf) -> Result<()> {
        let f = File::create(path)?;
        let mut w = BufWriter::new(f);
        serde_json::to_writer_pretty(&mut w, self)?;
        w.flush()?;
        Ok(())
    }

    fn colour(&self) -> Option<String> {
        inventory::iter::<ModuleRegistration>()
            .find_map(|e| self.modules.get(e.key).and_then(|v| (e.colour)(v)))
    }
}

impl From<&Machine> for Array {
    fn from(value: &Machine) -> Self {
        let mut array = Array::new();
        array.set_colour(value.colour());

        let mut entries: Vec<&ModuleRegistration> =
            inventory::iter::<ModuleRegistration>().collect();
        entries.sort_by_key(|e| e.priority);

        for entry in entries {
            if let Some(val) = value.modules.get(entry.key) {
                if let Some(lines) = (entry.display)(val.clone()) {
                    array.push_multi(lines);
                }
            }
        }
        array
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let array = Array::from(self);
        write!(f, "{array}")?;
        if self.colour_blocks {
            write!(
                f,
                "\n{}",
                colourblocks(array.get_indent() + SEPARATOR.len(), 16, 8)
            )?;
        }
        Ok(())
    }
}
