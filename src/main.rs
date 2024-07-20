use std::{env, fs, process};
fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Get the file path from the command line arguments
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("參數錯誤: {}", err);
        process::exit(1);
    });

    // Read the file
    let time_log = fs::read_to_string(config.file_path).unwrap_or_else(|err| {
        println!("讀取檔案時發生錯誤: {}", err);
        process::exit(2);
    });

    println!("檔案內容:\n{}", time_log);
}

struct Config {
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            1 => {
                println!("請指定要讀取的文字檔案，可將檔案拖曳到視窗\n檔案路徑:");
                // get input string from user
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).unwrap();
                Ok(Config { file_path: buf })
            }
            2 => Ok(Config {
                file_path: args[1].clone(),
            }),
            _ => Err("只能指定一個檔案路徑喔!"),
        }
    }
}
