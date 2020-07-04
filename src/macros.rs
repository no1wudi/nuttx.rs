#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::print_to_stdout($crate::format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::io::print_to_stdout(format_args!("\n"))
    };
    ($s:expr) => {
        $crate::io::print_to_stdout(format_args!(concat!($s, "\n")))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::io::print_to_stdout(format_args!(concat!($s, "\n"), $($tt)*))
    };
}
