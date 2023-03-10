mod kernel;
use crate::kernel::Kernel;

mod cpu;
use crate::cpu::Cpu;

mod mem;
use crate::mem::Memory;

fn main() {
    let kernel_info = Kernel::new();
    let cpu_info = Cpu::new();
    let mem_info = Memory::new();
    println!("Kernel: {}", kernel_info);
    println!("CPU: {}", cpu_info);
    println!("Memory: {}", mem_info);
}
