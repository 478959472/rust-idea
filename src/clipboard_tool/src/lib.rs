
pub mod str_tool {
    use clipboard_win::{get_clipboard_string, set_clipboard_string};
    pub fn replace_clipboard() {
        // 读取剪切板内容
        let content = get_clipboard_string().unwrap();

        // 替换"\"为"/"
        let replaced_str = content.replace("\\", "/");

        // 复制替换后的文本回剪切板
        set_clipboard_string(&replaced_str).unwrap();
    }
}
