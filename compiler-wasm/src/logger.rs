//! Logging functions using a logger object passed in from JS
use std::cell::RefCell;

use js_sys::Function;
use log::{Level, LevelFilter, Log, Metadata, Record};
use wasm_bindgen::prelude::*;

use crate::interop;

thread_local! {
    static INFO_FN: RefCell<Function> = RefCell::new(interop::stub_function());
}
thread_local! {
    static WARN_FN: RefCell<Function> = RefCell::new(interop::stub_function());
}
thread_local! {
    static ERROR_FN: RefCell<Function> = RefCell::new(interop::stub_function());
}

const LOGGER: Logger = Logger;

/// Bind the logger
///
/// The object passed in must have `info`, `warn` and `error` methods
pub fn bind(info_fn: Function, warn_fn: Function, error_fn: Function) -> Result<(), JsValue> {
    INFO_FN.replace(info_fn);
    WARN_FN.replace(warn_fn);
    ERROR_FN.replace(error_fn);

    if log::set_logger(&LOGGER).is_ok() {
        #[cfg(debug_assertions)]
        log::set_max_level(LevelFilter::Debug);
        #[cfg(not(debug_assertions))]
        log::set_max_level(LevelFilter::Info);
    }
    Ok(())
}

macro_rules! log_raw {
    ($fn:ident, $value:ident) => {
        $fn.with_borrow(|func| {
            let _ = func.call1(&JsValue::UNDEFINED, $value);
        });
    };
}

/// Log a raw object using logger with level `info`
#[inline]
pub fn raw_info(value: &JsValue) {
    log_raw!(INFO_FN, value);
}

/// Log a raw object using logger with level `warn`
#[inline]
pub fn raw_warn(value: &JsValue) {
    log_raw!(WARN_FN, value);
}

/// Log a raw object using logger with level `error`
#[inline]
pub fn raw_error(value: &JsValue) {
    log_raw!(ERROR_FN, value);
}

/// A placeholder struct to implement the `Log` trait for macros from the `log ` crate
struct Logger;
impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn flush(&self) {
        // nothing to do
    }

    fn log(&self, record: &Record) {
        let func = match record.level() {
            Level::Info => raw_info,
            Level::Warn => raw_warn,
            Level::Error => raw_error,
            _ => {
                #[cfg(debug_assertions)]
                {
                    raw_info
                }
                #[cfg(not(debug_assertions))]
                return;
            }
        };
        let log_value: JsValue = record.args().to_string().into();
        func(&log_value);
    }
}
