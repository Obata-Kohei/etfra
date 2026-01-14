use std::fs::OpenOptions;
use std::io::Write;

pub fn debug_log(msg: &str) {
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug.log")
        .unwrap();
    writeln!(f, "{}", msg).unwrap();
}
