use std::io::{self, Write};

#[derive(Clone, Debug)]
pub struct OutputItem {
    pub summary: String,
    pub details: String,
}

pub fn post_execution_menu(items: &[OutputItem]) -> Result<(), Box<dyn std::error::Error>> {
    if items.is_empty() {
        return Ok(());
    }

    loop {
        println!("\n----------------------------------");
        println!("[1] Search within results");
        println!("[2] Close (Back to main menu)");
        print!("Choice [1-2] > ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                print!("Enter search term: ");
                io::stdout().flush()?;

                let mut term_input = String::new();
                io::stdin().read_line(&mut term_input)?;
                let query = term_input.trim().to_lowercase();

                if query.is_empty() {
                    continue;
                }

                let matches: Vec<&OutputItem> = items
                    .iter()
                    .filter(|item| {
                        item.summary.to_lowercase().contains(&query)
                            || item.details.to_lowercase().contains(&query)
                    })
                    .collect();

                if matches.is_empty() {
                    println!("\nNo matches found for '{}'.", query);
                    continue;
                }

                println!("\nFound {} match(es):", matches.len());
                for (idx, item) in matches.iter().enumerate() {
                    println!("[{}] {}", idx + 1, item.summary);
                }

                print!("\nEnter number [1-{}] for details (or press Enter to skip): ", matches.len());
                io::stdout().flush()?;

                let mut choice_input = String::new();
                io::stdin().read_line(&mut choice_input)?;

                if let Ok(num) = choice_input.trim().parse::<usize>() {
                    if num >= 1 && num <= matches.len() {
                        println!("\n=== DETAILS ===");
                        println!("{}", matches[num - 1].details);
                    }
                }
            }
            "2" => break,
            _ => println!("Invalid selection. Please enter 1 or 2."),
        }
    }

    Ok(())
}