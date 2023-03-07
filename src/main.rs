use nix::sys::utsname::{UtsName, uname};
fn main() {
    println!("Hello, world!");
    let info = uname().unwrap();
    println!("{} {}", info.sysname().to_str().unwrap(), info.release().to_str().unwrap());
}
