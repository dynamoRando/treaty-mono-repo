use js_sys::Date;
use web_sys::console;

/// Takes a message and writes it to the browser console with a datetime stamp
pub fn log_to_console(message: String) {
    let now = Date::new(&Date::new_0());
    let now = Date::to_iso_string(&now);
    let message = format!("{}: {}", now.to_string(), &message);
    console::log_1(&message.into());
}
