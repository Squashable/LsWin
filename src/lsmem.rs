use serde::Deserialize;
use wmi::WMIConnection;
use crate::menu::{post_execution_menu, OutputItem};

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_PhysicalMemory", rename_all = "PascalCase")]
struct Win32PhysicalMemory {
    bank_label: Option<String>,
    capacity: Option<u64>,
    speed: Option<u32>,
    manufacturer: Option<String>,
    part_number: Option<String>,
}

pub fn run(wmi_con: &WMIConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== LSMEM ===");
    let modules: Vec<Win32PhysicalMemory> = wmi_con.query()?;
    let mut items = Vec::new();

    for ram in modules {
        let bank = ram.bank_label.unwrap_or_else(|| "Unknown Slot".to_string());
        let capacity_gb = ram.capacity.unwrap_or(0) / (1024 * 1024 * 1024);
        let speed = ram.speed.unwrap_or(0);
        let mfg = ram.manufacturer.unwrap_or_else(|| "Unknown Vendor".to_string()).trim().to_string();
        let part = ram.part_number.unwrap_or_else(|| "N/A".to_string()).trim().to_string();

        let summary = format!("{} | {} GB @ {} MHz | {}", bank, capacity_gb, speed, mfg);
        let details = format!(
            "Slot: {}\nCapacity: {} GB\nSpeed: {} MHz\nManufacturer: {}\nPart Number: {}",
            bank, capacity_gb, speed, mfg, part
        );

        println!("{}", summary);
        items.push(OutputItem { summary, details });
    }

    post_execution_menu(&items)?;
    Ok(())
}