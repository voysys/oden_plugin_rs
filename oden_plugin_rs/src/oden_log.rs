use crate::plugin_h::{self, OdenLogLevel};
use crate::plugin_h::{
    OdenLogLevel_e_OdenLogDebug, OdenLogLevel_e_OdenLogError, OdenLogLevel_e_OdenLogInfo,
    OdenLogLevel_e_OdenLogTrace, OdenLogLevel_e_OdenLogWarning,
};
use crate::ODEN_GLOBAL;
use log::{Level, Record};
use std::ffi::CString;

struct OdenLogger;

static ODEN_LOGGER: OdenLogger = OdenLogger;

impl log::Log for OdenLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
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

pub fn log(level: OdenLogLevel, file: &str, line: i32, module: &str, message: &str) {
    if unsafe { ODEN_GLOBAL.is_null() } {
        return;
    }
    let globals: &plugin_h::OdenPluginGlobalFunctions = unsafe { &*ODEN_GLOBAL };

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
