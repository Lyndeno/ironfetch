use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::colourblocks::colourblocks;

use crate::fetch::{Array, SEPARATOR};
use crate::modules::{
    battery::Battery, cpu::Cpu, disk::Disk, hostname::HostName, kernel::Kernel, memory::Memory,
    model::Model, osinfo::OsInfo, platform::Profile, ppd::Ppd, shell::Shell, uptime::Uptime,
};

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
    pub battery: Option<Battery>,
    pub ppd: Option<Ppd>,
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

    pub fn colour(&self) -> Option<String> {
        self.os.clone().and_then(|x| x.color)
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            kernel: Kernel::new().ok().flatten(),
            cpu: Cpu::new().ok(),
            memory: Memory::new().ok(),
            os: OsInfo::new().ok(),
            hostname: HostName::new().ok(),
            uptime: Uptime::new().ok(),
            model: Model::new().ok(),
            shell: Shell::new().ok(),
            platform: Profile::new().ok(),
            disk: Disk::new().ok(),
            battery: Battery::new().ok().flatten(),
            ppd: Ppd::new().ok(),
        }
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let array = Array::from(self.clone());

        write!(
            f,
            "{}\n{}",
            array,
            colourblocks(array.get_indent() + SEPARATOR.len(), 16, 8)
        )
    }
}

impl From<Machine> for Array {
    fn from(value: Machine) -> Self {
        let mut array = Array::new();
        array.set_colour(value.colour());

        array.push_obj_opt(value.os);
        array.push_obj_opt(value.shell);
        array.push_obj_opt(value.kernel);
        array.push_obj_opt(value.model);
        array.push_obj_opt(value.hostname);
        array.push_obj_opt(value.uptime);
        array.push_obj_opt(value.cpu);
        array.push_obj_opt(value.memory);
        array.push_obj_opt(value.platform);
        array.push_obj_opt(value.ppd);
        array.push_obj_opt(value.disk);
        array.push_obj_opt(value.battery);

        array
    }
}
