mod kernel;
use crate::kernel::Kernel;

mod cpu;
use crate::cpu::Cpu;

mod fetchline;
fn main() {
    println!("Hello, world!");
    let kernel_info = Kernel::new();
    let cpu_info = Cpu::new();
    println!("Kernel: {}", kernel_info);
    println!("CPU: {}", cpu_info);
}
