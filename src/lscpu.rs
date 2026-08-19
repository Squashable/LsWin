use serde::Deserialize;
use wmi::WMIConnection;
use crate::menu::{post_execution_menu, OutputItem};

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_Processor", rename_all = "PascalCase")]
struct Win32Processor {
    name: Option<String>,
    manufacturer: Option<String>,
    number_of_cores: Option<u32>,
    number_of_logical_processors: Option<u32>,
    max_clock_speed: Option<u32>,
}

pub fn run(wmi_con: &WMIConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== LSCPU ===");
    let processors: Vec<Win32Processor> = wmi_con.query()?;

    let mut items = Vec::new();

    for cpu in processors {
        let name = cpu.name.as_deref().unwrap_or("Unknown CPU").trim().to_string();
        let mfg = cpu.manufacturer.unwrap_or_else(|| "Unknown".to_string());
        let cores = cpu.number_of_cores.unwrap_or(0);
        let logical = cpu.number_of_logical_processors.unwrap_or(0);
        let speed = cpu.max_clock_speed.unwrap_or(0);

        let summary = format!("{} | {} Cores ({} Logical) | {} MHz", name, cores, logical, speed);
        let details = format!(
            "Model: {}\nVendor: {}\nCores: {} ({} Logical)\nMax Speed: {} MHz",
            name, mfg, cores, logical, speed
        );

        println!("{}", summary);
        items.push(OutputItem { summary, details });
    }

    post_execution_menu(&items)?;
    Ok(())
}