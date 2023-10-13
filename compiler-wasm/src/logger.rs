use std::cell::RefCell;

use js_sys::{Function, Reflect};
use log::{Level, Log, Metadata, Record, LevelFilter};
use wasm_bindgen::prelude::*;

use crate::wasm::stub_function;
thread_local! {
    static LOGGER_OBJ: RefCell<JsValue> = RefCell::new(JsValue::null());
}

thread_local! {
    static INFO_FN: RefCell<Function> = RefCell::new(stub_function());
}
thread_local! {
    static WARN_FN: RefCell<Function> = RefCell::new(stub_function());
}
thread_local! {
    static ERROR_FN: RefCell<Function> = RefCell::new(stub_function());
}
thread_local! {
    static DEBUG_FN: RefCell<Function> = RefCell::new(stub_function());
}

const LOGGER: Logger = Logger;

pub fn bind_logger(value: JsValue) -> Result<(), JsValue> {
    let info = Reflect::get(&value, &"info".into())?.dyn_into::<Function>()?;
    INFO_FN.replace(info);
    let warn = Reflect::get(&value, &"warn".into())?.dyn_into::<Function>()?;
    WARN_FN.replace(warn);
    let error = Reflect::get(&value, &"error".into())?.dyn_into::<Function>()?;
    ERROR_FN.replace(error);
    #[cfg(debug_assertions)]
    {
        let debug = Reflect::get(&value, &"debug".into())?.dyn_into::<Function>()?;
        DEBUG_FN.replace(debug);
    }
    LOGGER_OBJ.replace(value);

    if log::set_logger(&LOGGER).is_ok() {
        #[cfg(debug_assertions)]
        log::set_max_level(LevelFilter::Debug);
        #[cfg(not(debug_assertions))]
        log::set_max_level(LevelFilter::Info);
    }
    Ok(())
}

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn flush(&self) {
        // nothing to do
    }

    fn log(&self, record: &Record) {
        LOGGER_OBJ.with(|logger_this| {
            let log_value: JsValue = record.args().to_string().into();
            let func = match record.level() {
                Level::Info => &INFO_FN,
                Level::Warn => &WARN_FN,
                Level::Error => &ERROR_FN,
                _ => {
                    console::debug_1(&format!("[com] {}", record.args()).into());
                    return;
                }
            };
            func.with(|func| {
                let _ = func.borrow().call1(&logger_this.borrow(), &log_value);
            });
        })
    }
}
