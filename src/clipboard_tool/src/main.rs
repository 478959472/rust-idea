use clipboard_tool::str_tool;
use std::env;

// C:\Users\47895\Desktop\open_as_app.ps1
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: ./clipboard-tool --op-type log --log-path C:/Users/47895/iCloudDrive/Documents/clipboard_log");
        println!("Usage: ./clipboard-tool --op-type format-win-path");
        return;
    }
    let mut op_type = String::from("");
    let mut log_path = String::from("");

    for i in 1..args.len() {
        match args[i].as_str() {
            "--op-type" => {
                op_type = args[i + 1].clone();
            }
            "--log-path" => {
                log_path = args[i + 1].clone();
            }
            _ => {}
        }
    }
    println!("-op-type: {}", op_type);
    println!("-log-path: {}", log_path);
    if "log".eq_ignore_ascii_case(&op_type) {
        if args.len() < 5 {
            println!("Usage: ./clipboard-tool --op-type log --log-path C:/Users/47895/iCloudDrive/Documents/clipboard_log");
            return;
        }
        str_tool::log_clipboard(&log_path).expect("Failed to save clipboard");
    } else if "format-win-path".eq_ignore_ascii_case(&op_type) {
        str_tool::replace_clipboard();
    } else {
        println!("incorrect input parameters")
    }
}
