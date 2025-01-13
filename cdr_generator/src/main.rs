use chrono::{Utc, Duration};
use rand::Rng;
use serde::{Serialize, Deserialize};
use csv;
use std::fs;
use serde_json;

// Define the CDR structure
#[derive(Serialize, Deserialize, Debug)]
struct Cdr {
    call_id: u64,
    calling_number: String,
    called_number: String,
    start_time: String,
    end_time: String,
    duration: u32,
    call_type: String,
}

// Implement methods for the CDR struct
impl Cdr {
    fn new(call_id: u64) -> Self {
        let mut rng = rand::thread_rng();

        let calling_number = format!("21650{}", rng.gen_range(100000..999999));
        let called_number = format!("216{}", rng.gen_range(100000..999999));
        
        let now = Utc::now().naive_utc();
        let random_seconds_ago = rng.gen_range(0..(2 * 60 * 60));
        let start_time = now - Duration::seconds(random_seconds_ago as i64);
        
        let duration = rng.gen_range(1..3601);
        let end_time = start_time + Duration::seconds(duration as i64);

        let call_type = if rng.gen_bool(0.5) { "Incoming" } else { "Outgoing" };

        Cdr {
            call_id,
            calling_number,
            called_number,
            start_time: start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            end_time: end_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            duration,
            call_type: call_type.to_string(),
        }
    }
}

// Generate a specified number of CDRs
fn generate_cdrs(n: u64) -> Vec<Cdr> {
    (1..=n).map(Cdr::new).collect()
}

// Generate dynamic file name based on current timestamp
fn generate_file_name(prefix: &str) -> String {
    let now = Utc::now();
    format!("{prefix}{}", now.format("%Y%m%d%H%M%S"))
}

// Write CDRs to CSV
fn write_to_csv(cdrs: &[Cdr]) -> Result<(), Box<dyn std::error::Error>> {
    let dir = "OUTPUT/";
    fs::create_dir_all(dir)?;

    let file_name = dir.to_string() + &generate_file_name("CSV") + ".csv"; 
    let mut wtr = csv::Writer::from_path(&file_name)?;
    
    for cdr in cdrs {
        wtr.serialize(cdr)?;
    }
    
    wtr.flush()?;
    Ok(())
}

// Write CDRs to JSON
fn write_to_json(cdrs: &[Cdr]) -> Result<(), Box<dyn std::error::Error>> {
    let dir = "OUTPUT/";
    fs::create_dir_all(dir)?;

    let file_name = dir.to_string() + &generate_file_name("JSON") + ".json"; 
    let json_data = serde_json::to_string(cdrs)?;
    fs::write(file_name, json_data)?;
    Ok(())
}

// Main function to generate and write CDRs to CSV and JSON
fn main() {
    let cdrs = generate_cdrs(1000);
    if let Err(e) = write_to_csv(&cdrs) {
        eprintln!("Error writing to CSV: {}", e);
    }
    if let Err(e) = write_to_json(&cdrs) {
        eprintln!("Error writing to JSON: {}", e);
    }
}