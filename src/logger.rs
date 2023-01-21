use std::io::Write;

pub fn init() {
    let mut builder = env_logger::Builder::new();
    builder.filter_level(log::LevelFilter::Off);

    builder.filter_module(
        "revi_verifier",
        if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        },
    );

    builder.format(|buf, record| {
        let style = buf.style();

        let color = match record.level() {
            log::Level::Info => "\x1b[38;5;229mINFO\x1b[0m".to_string(),
            log::Level::Debug => "\x1b[38;5;5mDEBUG\x1b[0m".to_string(),
            _ => "UNKNOWN".to_string(),
        };

        writeln!(
            buf,
            "[{}] [{}]: {}",
            chrono::Local::now().format("%H:%M:%S"),
            color,
            style.value(record.args())
        )
    });

    builder.init();
}

