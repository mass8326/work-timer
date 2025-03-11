use std::env::current_dir;

use directories::ProjectDirs;
use flexi_logger::{FileSpec, FormatFunction, LogSpecBuilder, Logger, WriteMode};
use log::LevelFilter;

pub fn init() {
    let maybe_dir = ProjectDirs::from("dev", "reyma", "work-timer")
        .map_or_else(current_dir, |dirs| Ok(dirs.cache_dir().to_owned()));
    if let Ok(dir) = maybe_dir {
        let filespec = FileSpec::default()
            .directory(dir)
            .basename("work-timer")
            .suffix("log");
        let logspec = LogSpecBuilder::new()
            .module("work_timer", LevelFilter::Info)
            .build();
        let formatter: FormatFunction = |writer, now, record| {
            write!(
                writer,
                "[{}] [{}] {}",
                now.format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args(),
            )
        };
        let _ = Logger::with(logspec)
            .log_to_file(filespec)
            .write_mode(WriteMode::BufferAndFlush)
            .format(formatter)
            .start();
    }
}
