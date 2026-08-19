use std::io::{self, Write};

#[derive(Debug)]
pub enum Command {
    Lspci,
    Lsusb,
    Lscpu,
    Lsdrv,
    Lscrash,
    Lsdisk,
    Lsmem,
}

pub fn usr_input() -> Command {
    loop {
        let mut input = String::new();
        print!("> ");
        let _ = io::stdout().flush();

        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().to_lowercase().as_str() {
            "lspci" => return Command::Lspci,
            "lsusb" => return Command::Lsusb,
            "lscpu" => return Command::Lscpu,
            "lsdrv" => return Command::Lsdrv,
            "lscrash" => return Command::Lscrash,
            "lsdisk" => return Command::Lsdisk,
            "lsmem" => return Command::Lsmem,
            _ => {
                println!(
                    "Invalid command. Available options:\n\
                     - lspci   : List PCI / PCIe devices\n\
                     - lsusb   : List USB devices\n\
                     - lscpu   : Display CPU specifications and cores\n\
                     - lsdrv   : List installed kernel drivers\n\
                     - lscrash : View recent BSOD and sudden shutdown logs\n\
                     - lsdisk  : View storage drives and SMART health\n\
                     - lsmem   : View physical RAM topology and slots"
                );
            }
        }
    }
}