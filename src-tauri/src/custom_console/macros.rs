macro_rules! regular {
    ($($arg:tt)*) => { { unsafe{ crate::custom_console::buffer_queue::BUFFER.regular(format!("{}:{} - ", file!(), line!()) + &format!($($arg)*)); } } };
}
macro_rules! good {
    ($($arg:tt)*) => { { unsafe{ crate::custom_console::buffer_queue::BUFFER.good(format!("{}:{} - ", file!(), line!()) + &format!($($arg)*)); } } };
}
macro_rules! alert {
    ($($arg:tt)*) => { { unsafe{ crate::custom_console::buffer_queue::BUFFER.alert(format!("{}:{} - ", file!(), line!()) + &format!($($arg)*)); } } };
}
macro_rules! bad {
    ($($arg:tt)*) => { { unsafe{ crate::custom_console::buffer_queue::BUFFER.bad(format!("{}:{} - ", file!(), line!()) + &format!($($arg)*)); } } };
}
macro_rules! extract {
    () => { { unsafe{ crate::custom_console::buffer_queue::BUFFER.extract() } } };
}
            
pub(crate) use regular;
pub(crate) use good;
pub(crate) use alert;
pub(crate) use bad;
pub(crate) use extract;