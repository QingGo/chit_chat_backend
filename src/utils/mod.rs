macro_rules! format_and_debug_msg {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*).to_string();
        debug!("{}", message);
        message
    }}
}
pub(crate) use format_and_debug_msg;
