use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::colourblocks::colourblocks;

use crate::cpu::Cpu;
use crate::disk::Disk;
use crate::fetcherror::FetchError;
use crate::fetchsection::{FetchArray, SEPARATOR};
use crate::hostname::HostName;
use crate::kernel::Kernel;
use crate::memory::Memory;
use crate::model::Model;
use crate::osinfo::OsInfo;
use crate::platform::Profile;
use crate::shell::Shell;
use crate::uptime::Uptime;

use serde::{Deserialize, Serialize};

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

    pub fn from_file(path: PathBuf) -> Result<Self, FetchError> {
        let f = File::open(path)?;
        let r = BufReader::new(f);
        Ok(serde_json::from_reader(r)?)
    }

    pub fn to_file(&self, path: PathBuf) -> Result<(), FetchError> {
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
            memory: Memory::new(None).ok(),
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
        let mut array = FetchArray::default();

        if let Some(r) = &self.os {
            array.set_colour(r.color.clone());
            array.push(("OS", r));
        }

        if let Some(r) = &self.shell {
            array.push(("Shell", r));
        }

        if let Some(r) = &self.kernel {
            array.push(("Kernel", r));
        }

        if let Some(r) = &self.model {
            array.push(("Model", r));
        }

        if let Some(r) = &self.hostname {
            array.push(("Hostname", r));
        }

        if let Some(r) = &self.uptime {
            array.push(("Uptime", r));
        }

        if let Some(r) = &self.cpu {
            array.push(("CPU", r));
        }

        if let Some(r) = &self.memory {
            array.push(("Memory", &r));
            array.push(("Swap", r.display_swap()));
        }

        if let Some(r) = &self.platform {
            array.push(("Profile", r));
        }

        if let Some(r) = &self.disk {
            array.push(("Disk", r));
        }

        write!(
            f,
            "{}\n{}",
            array,
            colourblocks(array.get_indent() + SEPARATOR.len(), 16, 8)
        )
    }
}
