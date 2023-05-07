use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("C:/Users/47895/workspace/github/python_cy/temp.srt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);
}
