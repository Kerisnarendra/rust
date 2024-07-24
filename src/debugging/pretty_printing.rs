fn main() {
    let x = ();
    println!("The value of x is: {}", x);
}

// Output will be:
// Compiling rust v0.1.0 (/home/keris/rust/rust)
// error[E0277]: `()` doesn't implement `std::fmt::Display`
//  --> src/debugging/pretty_printing.rs:3:39
//   |
// 3 |     println!("The value of x is: {}", x);
//   |                                       ^ `()` cannot be formatted with the default formatter
//   |
//   = help: the trait `std::fmt::Display` is not implemented for `()`
//   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
//   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

// For more information about this error, try `rustc --explain E0277`.
// error: could not compile `rust` (bin "pretty_printing") due to 1 previous error
