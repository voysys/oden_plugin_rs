use crate::oden_global;
use crate::plugin_h::OdenLogLevel;
use crate::plugin_h::{
    OdenLogLevel_e_OdenLogDebug, OdenLogLevel_e_OdenLogError, OdenLogLevel_e_OdenLogInfo,
    OdenLogLevel_e_OdenLogTrace, OdenLogLevel_e_OdenLogWarning,
};
use log::{Level, LevelFilter, Record};
use std::ffi::CString;

struct OdenLogger;

static ODEN_LOGGER: OdenLogger = OdenLogger;

impl log::Log for OdenLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= oden_max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let file = record.file().unwrap_or("");
        let line = record.line().unwrap_or(0);
        let module = record.module_path().unwrap_or("");
        let message = format!("{}", record.args());

        match record.level() {
            Level::Error => log(
                OdenLogLevel_e_OdenLogError,
                file,
                line as i32,
                module,
                &message,
            ),
            Level::Warn => log(
                OdenLogLevel_e_OdenLogWarning,
                file,
                line as i32,
                module,
                &message,
            ),
            Level::Info => log(
                OdenLogLevel_e_OdenLogInfo,
                file,
                line as i32,
                module,
                &message,
            ),
            Level::Debug => log(
                OdenLogLevel_e_OdenLogDebug,
                file,
                line as i32,
                module,
                &message,
            ),
            Level::Trace => log(
                OdenLogLevel_e_OdenLogTrace,
                file,
                line as i32,
                module,
                &message,
            ),
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_max_level(log::LevelFilter::Trace);
    log::set_logger(&ODEN_LOGGER)
}

fn oden_max_level() -> LevelFilter {
    let Some(oden_log_max_level) = oden_global().and_then(|globals| globals.logMaxLevel) else {
        return LevelFilter::Trace;
    };

    #[allow(non_upper_case_globals)]
    match unsafe { oden_log_max_level() } {
        OdenLogLevel_e_OdenLogTrace => LevelFilter::Trace,
        OdenLogLevel_e_OdenLogDebug => LevelFilter::Debug,
        OdenLogLevel_e_OdenLogInfo => LevelFilter::Info,
        OdenLogLevel_e_OdenLogWarning => LevelFilter::Warn,
        OdenLogLevel_e_OdenLogError => LevelFilter::Error,
        _ => LevelFilter::Off,
    }
}

pub fn log(level: OdenLogLevel, file: &str, line: i32, module: &str, message: &str) {
    let Some(globals) = oden_global() else {
        return;
    };

    let c_file = CString::new(file.trim_end_matches('\0')).unwrap_or_else(|_| {
        CString::new("Invalid log file, file name has internal null bytes").unwrap()
    });

    let c_module = CString::new(module.trim_end_matches('\0'))
        .unwrap_or_else(|_| CString::new("Invalid log tag, tag has internal null bytes").unwrap());

    let c_message = CString::new(message.trim_end_matches('\0')).unwrap_or_else(|_| {
        CString::new("Invalid log message, message has internal null bytes").unwrap()
    });

    if let Some(oden_log_raw) = globals.logRaw {
        unsafe {
            oden_log_raw(
                level,
                c_file.as_ptr() as *const _,
                std::ptr::null(),
                line,
                c_module.as_ptr() as *const _,
                c"%s".as_ptr() as *const _,
                c_message.as_ptr() as *const _,
            )
        };
    }
}
