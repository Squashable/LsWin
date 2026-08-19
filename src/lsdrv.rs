use serde::Deserialize;
use wmi::WMIConnection;
use crate::menu::{post_execution_menu, OutputItem};

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_SystemDriver", rename_all = "PascalCase")]
struct Win32SystemDriver {
    name: Option<String>,
    state: Option<String>,
    start_mode: Option<String>,
    path_name: Option<String>,
}

pub fn run(wmi_con: &WMIConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== LSDRV ===");
    let drivers: Vec<Win32SystemDriver> = wmi_con.query()?;

    let mut items = Vec::new();

    for drv in drivers {
        let name = drv.name.unwrap_or_else(|| "Unknown".to_string());
        let state = drv.state.unwrap_or_else(|| "N/A".to_string());
        let start = drv.start_mode.unwrap_or_else(|| "N/A".to_string());
        let path = drv.path_name.unwrap_or_else(|| "N/A".to_string());

        let summary = format!("{:<28} | {:<10} | {:<12}", name, state, start);
        let details = format!("Driver Name: {}\nState: {}\nStart Mode: {}\nPath: {}", name, state, start, path);

        println!("{}", summary);
        items.push(OutputItem { summary, details });
    }

    post_execution_menu(&items)?;
    Ok(())
}