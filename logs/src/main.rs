use std::fs;
use std::io::Error;

fn extract_log(text:&str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }
    results
}

fn main1() {
    //method - 1
    let mut errors = vec![];

    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            println!("{}", text.len());
            errors = extract_log(text.as_str());
        }
        Err(reason) => {
            println!("Failed to read due to : {}", reason);
        }
    }
    
    match fs::write("errorlogs.txt", errors.join("\n")) {
        Ok(..) => {
            println!("Wrote error logs text");
        }
        Err(reason) => {
            println!("Writing error logs failed due to : {}", reason);
        }
    }

    println!("Errors : {:#?}", errors);

    //method - 2
    let text = fs::read_to_string("logs.txt").expect("failed to read logs.txt");
    let error_logs = extract_log(text.as_str());
    fs::write("errorlogs.txt", error_logs.join("\n")).expect("failed to write error logs");
}

//method - 3 using "?" (try)
fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_log(text.as_str());
    fs::write("errorlogs.txt", error_logs.join("\n"))?;

    Ok(())
}