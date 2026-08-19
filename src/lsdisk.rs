use serde::Deserialize;
use wmi::WMIConnection;
use crate::menu::{post_execution_menu, OutputItem};

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_DiskDrive", rename_all = "PascalCase")]
struct Win32DiskDrive {
    model: Option<String>,
    media_type: Option<String>,
    size: Option<u64>,
    status: Option<String>,
    interface_type: Option<String>,
}

pub fn run(wmi_con: &WMIConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== LSDISK ===");
    let drives: Vec<Win32DiskDrive> = wmi_con.query()?;
    let mut items = Vec::new();

    for disk in drives {
        let model = disk.model.unwrap_or_else(|| "Unknown Model".to_string());
        let status = disk.status.unwrap_or_else(|| "UNKNOWN".to_string());
        let media = disk.media_type.unwrap_or_else(|| "Unknown Media".to_string());
        let interface = disk.interface_type.unwrap_or_else(|| "N/A".to_string());
        let size_gb = disk.size.unwrap_or(0) / (1024 * 1024 * 1024);

        let summary = format!("{} | {} GB | Health: {}", model, size_gb, status);
        let details = format!(
            "Model: {}\nCapacity: {} GB\nSMART Status: {}\nMedia Type: {}\nInterface: {}",
            model, size_gb, status, media, interface
        );

        println!("{}", summary);
        items.push(OutputItem { summary, details });
    }

    post_execution_menu(&items)?;
    Ok(())
}