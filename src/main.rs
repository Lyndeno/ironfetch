mod kernel;
use crate::kernel::Kernel;

mod cpu;
use crate::cpu::Cpu;

mod mem;
use crate::mem::Memory;

use os_release;

fn main() {
    let kernel_info = Kernel::new();
    let cpu_info = Cpu::new();
    let mem_info = Memory::new();
    let os = os_release::OsRelease::new().unwrap();
    println!("Distro: {}", os.pretty_name);
    println!("Kernel: {}", kernel_info);
    println!("CPU: {}", cpu_info);
    println!("Memory: {}", mem_info);
    println!("OS: {}", os.pretty_name);
}
