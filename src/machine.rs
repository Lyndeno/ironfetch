use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::colourblocks::colourblocks;

use crate::cpu::Cpu;
use crate::disk::Disk;
use crate::fetcharray::FetchArray;
use crate::fetchsection::{FetchSection, SEPARATOR};
use crate::hostname::HostName;
use crate::kernel::Kernel;
use crate::memory::Memory;
use crate::model::Model;
use crate::osinfo::OsInfo;
use crate::platform::Profile;
use crate::shell::Shell;
use crate::uptime::Uptime;

use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Serialize, Deserialize, Clone)]
pub struct Machine {
    pub kernel: Option<Kernel>,
    pub cpu: Option<Cpu>,
    pub memory: Option<Memory>,
    pub os: Option<OsInfo>,
    pub hostname: Option<HostName>,
    pub uptime: Option<Uptime>,
    pub model: Option<Model>,
    pub shell: Option<Shell>,
    pub platform: Option<Profile>,
    pub disk: Option<Disk>,
}

impl Machine {
    pub fn new() -> Self {
        Self::default()
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
    /// Returns () if writing failed
    pub fn to_file(&self, path: PathBuf) -> Result<()> {
        let f = File::create(path)?;
        let mut w = BufWriter::new(f);
        serde_json::to_writer_pretty(&mut w, self)?;
        w.flush()?;
        Ok(())
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            kernel: Kernel::new().ok(),
            cpu: Cpu::new().ok(),
            memory: Memory::new().ok(),
            os: OsInfo::new().ok(),
            hostname: HostName::new().ok(),
            uptime: Uptime::new().ok(),
            model: Model::new().ok(),
            shell: Shell::new().ok(),
            platform: Profile::new().ok(),
            disk: Disk::new().ok(),
        }
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let array = FetchArray::from(self.clone());

        write!(
            f,
            "{}\n{}",
            array,
            colourblocks(array.get_indent() + SEPARATOR.len(), 16, 8)
        )
    }
}

impl From<Machine> for FetchArray {
    fn from(value: Machine) -> Self {
        let mut array: Vec<Result<FetchSection>> = Vec::new();

        let colour = if let Some(r) = value.os {
            array.push(Ok(r.clone().into()));
            r.color
        } else {
            None
        };

        array.push(value.shell.try_into());
        array.push(value.kernel.try_into());
        array.push(value.model.try_into());
        array.push(value.hostname.try_into());
        array.push(value.uptime.try_into());
        array.push(value.cpu.try_into());

        if let Some(r) = value.memory {
            let arr: Vec<FetchSection> = r.into();
            array.append(&mut arr.into_iter().map(Ok).collect());
        }

        array.push(value.platform.try_into());
        array.push(value.disk.try_into());

        let sections: Vec<FetchSection> = array.into_iter().flatten().collect();

        let mut fetch_array = Self::from(sections);
        fetch_array.set_colour(colour);
        fetch_array
    }
}
