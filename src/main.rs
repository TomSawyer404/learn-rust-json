use serde_derive::{Deserialize, Serialize};
use sysinfo::{System, SystemExt};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    city: String,
    server: String,
    time: String,
    info: String,
}

fn get_all_info(seq: &str) -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let host_name = sys.host_name().unwrap();
    let memory = format!(" memory:{}/{}", sys.used_memory(), sys.total_memory());
    let processors = format!(" processors:{}", sys.processors().len());

    let mut items = Vec::new();
    for item in [host_name, memory, processors] {
        items.push(item);
    }

    items.join(seq)
}

fn main() {
    // 1. Get related information from argv[]
    if std::env::args().len() != 4 {
        eprintln!(
            "Usage: {} <city_name> <server_name> <output_path>",
            std::env::args().nth(0).unwrap()
        );
        std::process::exit(1);
    }
    let input_city_name = std::env::args().nth(1).unwrap();
    let input_server_name = std::env::args().nth(2).unwrap();
    let output = std::env::args().nth(3).unwrap();
    let now = chrono::Utc::now().to_string();

    // 2. Config information into struct
    let out_json = Config {
        city: input_city_name,
        server: input_server_name,
        time: now,
        info: get_all_info(""),
    };

    // 3. Write to file
    std::fs::write(output, serde_json::to_string_pretty(&out_json).unwrap()).unwrap();
    println!("\tDone!");
}
