mod input;
mod lscpu;
mod lsdisk;
mod lsdrv;
mod lscrash;
mod lsmem;
mod lspci;
mod lsusb;
mod menu;

use input::{usr_input, Command};
use wmi::{COMLibrary, WMIConnection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con)?;

    loop {
        let result = match usr_input() {
            Command::Lspci => lspci::run(&wmi_con),
            Command::Lsusb => lsusb::run(&wmi_con),
            Command::Lscpu => lscpu::run(&wmi_con),
            Command::Lsdrv => lsdrv::run(&wmi_con),
            Command::Lscrash => lscrash::run(&wmi_con),
            Command::Lsdisk => lsdisk::run(&wmi_con),
            Command::Lsmem => lsmem::run(&wmi_con),
        };

        if let Err(e) = result {
            println!("\n[Error executing command]: {}", e);
        }

        println!();
    }
}