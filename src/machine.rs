use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::colourblocks::colourblocks;

use crate::cpu::Cpu;
use crate::disk::Disk;
use crate::fetcharray::FetchArray;
use crate::fetchsection::SEPARATOR;
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

#[derive(Serialize, Deserialize)]
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

    pub fn from_file(path: PathBuf) -> Result<Self> {
        let f = File::open(path)?;
        let r = BufReader::new(f);
        Ok(serde_json::from_reader(r)?)
    }

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
        let array = FetchArray::from(self);

        write!(
            f,
            "{}\n{}",
            array,
            colourblocks(array.get_indent() + SEPARATOR.len(), 16, 8)
        )
    }
}

impl From<&Machine> for FetchArray {
    fn from(value: &Machine) -> Self {
        let mut array = Self::new();

        if let Some(r) = &value.os {
            array.set_colour(r.color.clone());
            array.push(("OS", r));
        }

        if let Some(r) = &value.shell {
            array.push(("Shell", r));
        }

        if let Some(r) = &value.kernel {
            array.push(("Kernel", r));
        }

        if let Some(r) = &value.model {
            array.push(("Model", r));
        }

        if let Some(r) = &value.hostname {
            array.push(("Hostname", r));
        }

        if let Some(r) = &value.uptime {
            array.push(("Uptime", r));
        }

        if let Some(r) = &value.cpu {
            array.push(("CPU", r));
        }

        if let Some(r) = &value.memory {
            array.push(("Memory", &r));
            array.push(("Swap", r.display_swap()));
        }

        if let Some(r) = &value.platform {
            array.push(("Profile", r));
        }

        if let Some(r) = &value.disk {
            array.push(("Disk", r));
        }

        array
    }
}
