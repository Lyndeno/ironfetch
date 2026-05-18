use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::colourblocks::colourblocks;
use crate::fetch::{Array, ModuleRegistration, SEPARATOR};
use crate::Result;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Machine {
    // flatten collapses the map's key-value pairs into the top-level JSON
    // object, giving {"OS": {...}, "CPU": {...}} rather than {"modules": {...}}.
    #[serde(flatten)]
    modules: IndexMap<String, serde_json::Value>,
}

impl Machine {
    pub fn new() -> Self {
        // inventory::iter order is not guaranteed, so sort by priority to
        // preserve the intended display order across builds.
        let mut entries: Vec<&ModuleRegistration> =
            inventory::iter::<ModuleRegistration>().collect();
        entries.sort_by_key(|e| e.priority);

        Self {
            modules: entries
                .iter()
                .filter_map(|e| Some((e.key.to_string(), (e.load)()?)))
                .collect(),
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
        write!(
            f,
            "{}\n{}",
            array,
            colourblocks(array.get_indent() + SEPARATOR.len(), 16, 8)
        )
    }
}
