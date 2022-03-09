use std::convert::From;

#[derive(Debug)]
enum LogType {
    Replay,
    Record,
}

impl From<LogType> for &str {
    fn from(log_type: LogType) -> &'static str {
        match log_type {
            LogType::Replay => "REPLAY",
            LogType::Record => "RECORD",
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
