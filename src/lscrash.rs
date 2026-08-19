use serde::Deserialize;
use wmi::WMIConnection;
use crate::menu::{post_execution_menu, OutputItem};

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_NTLogEvent", rename_all = "PascalCase")]
struct Win32NTLogEvent {
    event_code: Option<u32>,
    message: Option<String>,
    time_generated: Option<String>,
}

pub fn run(wmi_con: &WMIConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== LSCRASH ===");
    let query = "SELECT EventCode, Message, TimeGenerated FROM Win32_NTLogEvent WHERE Logfile = 'System' AND (EventCode = 1001 OR EventCode = 41)";
    let events: Vec<Win32NTLogEvent> = wmi_con.raw_query(query)?;

    if events.is_empty() {
        println!("No recent BSOD BugChecks or dirty shutdowns found.");
        return Ok(());
    }

    let mut items = Vec::new();

    for ev in events {
        let code = ev.event_code.unwrap_or(0);
        let type_label = match code {
            1001 => "BSOD BugCheck (Event 1001)",
            41 => "Kernel-Power / Sudden Shutdown (Event 41)",
            _ => "Unknown Event",
        };
        let time = ev.time_generated.unwrap_or_else(|| "Unknown Time".to_string());
        let msg = ev.message.unwrap_or_else(|| "No details provided.".to_string());

        let summary = format!("{} | {}", type_label, time);
        let details = format!("Type: {}\nTime: {}\nMessage:\n{}", type_label, time, msg.trim());

        println!("{}", summary);
        items.push(OutputItem { summary, details });
    }

    post_execution_menu(&items)?;
    Ok(())
}