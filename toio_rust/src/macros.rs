// global debug flag, set once at startup from the CLI args
pub static DEBUG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// prints a line only when the debug flag is set
#[macro_export]
macro_rules! debugPrint {
    ($($arg:tt)*) => {
        if $crate::macros::DEBUG.load(std::sync::atomic::Ordering::Relaxed) {
            println!($($arg)*);
        }
    };
}

// builds a vector array of OSC i32
#[macro_export]
macro_rules! OscInts {
    ($($val:expr),* $(,)?) => {
        vec![$(OscType::Int($val as i32)),*]
    };
}

// returns "N/A" if expression doesn't match a pattern
#[macro_export]
macro_rules! or_na {
    ($pat:pat = $expr:expr => $body:expr) => {
        if let $pat = $expr {
            $body
        } else {
            "N/A".to_string()
        }
    };
}
