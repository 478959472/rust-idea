
pub mod str_tool {
    use clipboard_win::{get_clipboard_string, set_clipboard_string};
    use std::fs::OpenOptions;
    use std::io::{self, Write};
    use std::ops::Add;
    use chrono::prelude::*;
    use std::fs;
    use std::path::Path;

    pub fn replace_clipboard() {
        // 读取剪切板内容
        let content = get_clipboard_string().unwrap();

        // 替换"\"为"/"
        let replaced_str = content.replace("\\", "/");

        // 复制替换后的文本回剪切板
        set_clipboard_string(&replaced_str).unwrap();
    }


    pub fn log_clipboard(log_path: &str) -> io::Result<()> {
        create_path(log_path);
        let mut log_path = String::from(log_path);
        if !log_path.ends_with("/") && !log_path.ends_with("\\"){
            log_path.push('/')
        }
        // 读取剪切板内容
        let  clipboard_content = get_clipboard_string().unwrap();

        let mut content =format!("\r\n{}\r\n", Local::now().format("%Y-%m-%d %H:%M:%S") );
        content.push_str(&clipboard_content);
        // 获取当前日期
        let today =Local::now();

        let  filename = format!("{}.txt", today.format("%Y-%m-%d"));
        let  filename = log_path.add(&filename);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filename)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    fn create_path(file_path: &str){
        // 检查文件路径是否存在
        if Path::new(&file_path).exists() {
            println!("文件路径已存在");
        } else {
            // 创建完整路径
            if let Some(parent) = Path::new(&file_path).parent() {
                if !parent.exists() {
                    if let Err(err) = fs::create_dir_all(parent) {
                        eprintln!("无法创建路径: {}", err);
                        return;
                    }
                    println!("路径已创建");
                }
            }

            // 创建文件
            if let Err(err) = fs::File::create(&file_path) {
                eprintln!("无法创建文件: {}", err);
                return;
            }
            println!("文件已创建");
        }
    }
}
