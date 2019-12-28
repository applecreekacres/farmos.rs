use simplelog::*;
use std::fs::File;

/// Basic abstraction for initializing logging.
/// Currently using SimpleLog to log events.
/// The following macros are available:logging_init()
/// error!
/// info!
/// debug!
/// warn!
/// trace!
pub fn logging_init() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("farmos.log").unwrap(),
        ),
    ])
    .unwrap();
    info!("Logging initialized");
}
