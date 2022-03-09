use std::convert::From;

#[derive(Debug)]
enum LogType {
    Replay,
    Record,
    Invalid,
}

impl From<&str> for LogType {
    fn from(txt: &str) -> Self {
        match txt {
            "REPLAY" => LogType::Replay,
            "RECORD" => LogType::Record,
            _ => LogType::Invalid,
        }
    }
}

impl From<LogType> for &str {
    fn from(log_type: LogType) -> &'static str {
        match log_type {
            LogType::Replay => "REPLAY",
            LogType::Record => "RECORD",
            LogType::Invalid => "INVALID",
        }
    }
}

fn main() {
    let lt1 = LogType::Replay;
    let lt2 = LogType::Record;

    let foo1: &str = lt1.into();
    let foo2: &str = lt2.into();
    
    println!("foo1 = {}, foo2 = {}", foo1, foo2);
}
