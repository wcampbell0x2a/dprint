/// Prints to the standard ouput only in debug build.
/// In release build this macro is not compiled thanks to `#[cfg(debug_assertions)]`.
/// see [https://doc.rust-lang.org/std/macro.print.html](https://doc.rust-lang.org/std/macro.print.html) for more info.
#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] std::print!($($arg)*));
}

/// Prints to the standard ouput only in debug build.
/// In release build this macro is not compiled thanks to `#[cfg(debug_assertions)]`.
/// see [https://doc.rust-lang.org/std/macro.println.html](https://doc.rust-lang.org/std/macro.println.html) for more info.
#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] std::println!($($arg)*));
}

/// Prints to the standard error only in debug build.
/// In release build this macro is not compiled thanks to `#[cfg(debug_assertions)]`.
/// see [https://doc.rust-lang.org/std/macro.eprint.html](https://doc.rust-lang.org/std/macro.eprint.html) for more info.
#[macro_export]
macro_rules! deprint {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] std::eprint!($($arg)*));
}

/// Prints to the standard error only in debug build.
/// In release build this macro is not compiled thanks to `#[cfg(debug_assertions)]`.
/// see [https://doc.rust-lang.org/std/macro.eprintln.html](https://doc.rust-lang.org/std/macro.eprintln.html) for more info.
#[macro_export]
macro_rules! deprintln {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] std::eprintln!($($arg)*));
}
