use std::collections::HashMap;
use std::collections::HashSet;

use ironfetch::colourblocks::colourblocks;
use ironfetch::fetchsection::{FetchArray, SEPARATOR};
use ironfetch::kernel::Kernel;

use ironfetch::cpu::Cpu;

use ironfetch::memory::Memory;

use ironfetch::osinfo::OsInfo;

use ironfetch::hostname::HostName;

use ironfetch::uptime::Uptime;

use ironfetch::model::Model;

use ironfetch::shell::Shell;

use ironfetch::platform::Profile;

use futures::executor;

use clap::Parser;

use ironfetch::args::Args;

async fn test() {
    let client = udisks2::Client::new().await.unwrap();
    let manager = client.manager();
    let objects = manager.get_block_devices(HashMap::new()).await.unwrap();

    let mut v = Vec::new();
    for obj in objects {
        println!("{:?}", obj);
        v.push(obj.to_string());
    }

    println!("{:?}", v);
    //let set: HashSet<_> = v.drain(..).collect();
    //v.extend(set.into_iter());
    let mut hm = HashMap::new();
    for drivestr in v {
        let object = client.object(drivestr.clone());
        if let Ok(o) = object {
            let block = o.block().await;
            if let Ok(b) = block {
                let drive = client.drive_for_block(&b).await;
                if let Ok(d) = drive {
                    println!(
                        "{} {}: Size: {}",
                        drivestr,
                        d.id().await.unwrap(),
                        client.size_for_display(d.size().await.unwrap(), true, true)
                    );
                    hm.insert(d.id().await.unwrap(), d.size().await.unwrap());
                }
            }
        }
    }
    dbg!(hm);
}

fn main() {
    let args = Args::parse();

    let mut array = FetchArray::default();

    executor::block_on(test());

    if let Ok(r) = OsInfo::new() {
        array.set_colour(r.color());
        array.push(("OS", r));
    }

    if let Ok(r) = Shell::new() {
        array.push(("Shell", r));
    }

    if let Ok(r) = Kernel::new() {
        array.push(("Kernel", r));
    }

    if let Ok(r) = Model::new() {
        array.push(("Model", r));
    }

    if let Ok(r) = HostName::new() {
        array.push(("Hostname", r));
    }

    if let Ok(r) = Uptime::new() {
        array.push(("Uptime", r));
    }

    if let Ok(r) = Cpu::new() {
        array.push(("CPU", r));
    }

    if let Ok(r) = Memory::new(args.memory_unit) {
        array.push(("Memory", &r));
        array.push(("Swap", r.display_swap()));
    }

    if let Ok(r) = Profile::new() {
        array.push(("Profile", r));
    }

    println!(
        "{}\n{}",
        array,
        colourblocks(
            array.get_indent() + SEPARATOR.len(),
            args.colours,
            args.colour_length
        )
    );
}
