use serde::Deserialize;
use wmi::WMIConnection;
use crate::menu::{post_execution_menu, OutputItem};

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_PnPEntity", rename_all = "PascalCase")]
struct Win32PnPEntity {
    name: Option<String>,
    device_i_d: Option<String>,
    manufacturer: Option<String>,
}

pub fn run(wmi_con: &WMIConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== LSPCI ===");
    let pci_query = "SELECT Name, DeviceID, Manufacturer FROM Win32_PnPEntity WHERE DeviceID LIKE 'PCI%'";
    let pci_devices: Vec<Win32PnPEntity> = wmi_con.raw_query(pci_query)?;

    let mut items = Vec::new();

    for device in pci_devices {
        let dev_id = device.device_i_d.unwrap_or_else(|| "Unknown ID".to_string());
        let name = device.name.unwrap_or_else(|| "Unknown Device".to_string());
        let mfg = device.manufacturer.unwrap_or_else(|| "Unknown".to_string());

        let summary = format!("{} | {}", dev_id, name);
        let details = format!("Name: {}\nDevice ID: {}\nManufacturer: {}", name, dev_id, mfg);

        println!("{}", summary);
        items.push(OutputItem { summary, details });
    }

    post_execution_menu(&items)?;
    Ok(())
}