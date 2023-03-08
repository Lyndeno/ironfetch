mod kernel;
use crate::kernel::Kernel;
fn main() {
    println!("Hello, world!");
    let kernel_info = Kernel::new();
    println!("{}", kernel_info);
}
