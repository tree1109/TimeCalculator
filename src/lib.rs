pub mod time;
pub mod time_interval;
pub mod time_log;
pub mod time_parser;

// Re-export the modules
pub use time::Time;
pub use time_interval::TimeInterval;
pub use time_log::TimeLog;

use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the file
    let contents = fs::read_to_string(config.file_path)?;

    // Parse the time intervals
    let time_intervals = time_parser::get_time_interval_from_lines(contents.lines().collect());
    let time_log = TimeLog::new(time_intervals);

    // Print the time log and the total time
    let time_log_string = time_log.get_time_log_strings();
    let total_time = time_log.get_total_time_in_minutes();
    let hours = total_time / 60;
    let minutes = total_time % 60;
    println!("時間紀錄:\n{}", time_log_string);
    println!("總共時間:\n總共{}小時又{}分鐘~ UwU", hours, minutes);

    Ok(())
}

pub struct Config {
    file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            // If there is no file path, use the default file path
            1 => Ok(Config {
                file_path: String::from("time_log.txt"),
            }),
            // If there is a file path, use the file path from the command line arguments
            2 => Ok(Config {
                file_path: args[1].clone(),
            }),
            // If there are more than one file path, return an error
            _ => Err("只能指定一個檔案路徑喔"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let config = Config {
            file_path: String::from("time.txt"),
        };

        assert_eq!(run(config).is_ok(), true);
    }

    #[test]
    fn test_config_new() {
        let args = vec![String::from("summary_time_calculator")];
        assert!(Config::new(&args).is_ok());

        let args = vec![
            String::from("summary_time_calculator"),
            String::from("test_time_log.txt"),
        ];
        assert!(Config::new(&args).is_ok());

        let args = vec![
            String::from("summary_time_calculator"),
            String::from("test_time_log.txt"),
            String::from("test_time_log.txt"),
        ];
        assert!(Config::new(&args).is_err());

        let args = vec![];
        assert!(Config::new(&args).is_err());
    }
}
