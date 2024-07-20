pub mod time;
pub mod time_interval;
pub mod time_log;
pub mod time_parser;

pub use time::Time;
pub use time_interval::TimeInterval;
pub use time_log::TimeLog;

use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the file
    let contents = fs::read_to_string(config.file_path)?;

    println!("檔案內容:\n{}", contents);

    Ok(())
}

pub struct Config {
    file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            1 => {
                println!("請指定要讀取的文字檔案，可將檔案拖曳到視窗\n檔案路徑:");
                // get input string from user
                let mut buf = String::new();
                match std::io::stdin().read_line(&mut buf) {
                    Ok(_) => Ok(Config {
                        file_path: buf.trim().to_string(),
                    }),
                    Err(_) => Err("讀取輸入錯誤"),
                }
            }
            2 => Ok(Config {
                file_path: args[1].clone(),
            }),
            _ => Err("只能指定一個檔案路徑喔"),
        }
    }
}
