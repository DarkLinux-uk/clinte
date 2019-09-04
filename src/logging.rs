use std::fs;
use std::fs::File;

use chrono::offset::Utc;
use simplelog::*;
use users;

lazy_static! {
    static ref FILE: String = format!(
        "/tmp/clinte_{}.log",
        users::get_current_username()
            .unwrap()
            .into_string()
            .unwrap()
    );
}

pub fn init() {
    // If the log file exists on startup,
    // move and timestamp it so we get a
    // fresh log file.
    if fs::metadata(FILE.clone()).is_ok() {
        let mut new_file = FILE.clone();
        let time = Utc::now().to_rfc3339();
        new_file.push_str(".");
        new_file.push_str(&time);
        fs::rename(FILE.clone(), new_file).unwrap();
    }

    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Stderr).unwrap(),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(FILE.clone()).unwrap(),
        ),
    ])
    .expect("Unable to initialize logging");
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;

    #[test]
    fn init_logs() {
        let blank = " ".bytes().collect::<Vec<u8>>();
        fs::write("/tmp/clinte.log", &blank).unwrap();
        init();

        info!("TEST LOG MESSAGE");
        let logfile = fs::read_to_string("/tmp/clinte.log").unwrap();
        assert!(logfile.contains("TEST LOG MESSAGE"));
    }
}
