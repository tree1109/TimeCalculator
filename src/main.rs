use std::{env, process};

use summary_time_calculator::{run, Config};

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Get the file path from the command line arguments
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("參數錯誤: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("執行發生錯誤: {}", e);
        process::exit(2);
    }
}
