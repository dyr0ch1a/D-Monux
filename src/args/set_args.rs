use std::env;

pub enum Command {
    Init,
    Unknown,
}

pub fn parser_args() -> Command {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        Some("init") => Command::Init,
        _ => Command::Unknown,
    }
}
