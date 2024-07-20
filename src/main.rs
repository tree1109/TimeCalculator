use std::{env, fs};
fn main() {
    // Get the command line arguments
    let args: env::Args = env::args();

    // Get the file path from the command line arguments
    let file_path = match args.len() {
        1 => {
            println!("請指定要讀取的文字檔案，可將檔案拖曳到視窗\n檔案路徑:");
            // get input string from user
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input
        }
        2 => args.skip(1).next().unwrap(),
        _ => {
            println!("只能指定一個檔案路徑喔!");
            return;
        }
    };

    let time_log = fs::read_to_string(file_path.trim()).expect("無法讀取檔案!");

    println!("檔案內容:\n{}", time_log);
}
