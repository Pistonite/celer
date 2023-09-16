use log::{Log, Metadata, Record, Level};
use wasm_bindgen::JsValue;
use web_sys::console;

/// log crate implementation with console
pub struct Logger;

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn flush(&self) {
        // nothing to do
    }

    fn log(&self, record: &Record) {
        match record.level() {
            Level::Error => console::error_1(&format!("[com] {}", record.args()).into()),
            Level::Warn => console::warn_1(&format!("[com] {}", record.args()).into()),
            Level::Info => console::info_1(&format!("[com] {}", record.args()).into()),
            _ => console::debug_1(&format!("[com] {}", record.args()).into()),
        }
    }
}

