use std::{env, fs};
fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Get the file path from the command line arguments
    let config = Config::new(args);

    let time_log = fs::read_to_string(config.file_path).expect("無法讀取檔案!");

    println!("檔案內容:\n{}", time_log);
}

struct Config {
    file_path: String,
}

impl Config {
    fn new(args: Vec<String>) -> Config {
        let file_path = match args.len() {
            1 => {
                println!("請指定要讀取的文字檔案，可將檔案拖曳到視窗\n檔案路徑:");
                // get input string from user
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).unwrap();
                buf
            }
            2 => args[1].clone(),
            _ => {
                println!("只能指定一個檔案路徑喔!");
                String::new()
            }
        };
        Config { file_path }
    }
}
