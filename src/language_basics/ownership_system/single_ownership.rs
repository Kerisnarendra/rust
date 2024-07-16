fn main() {
    let data1 = String::from("Hello");
    let data2 = data1; // data1 is moved to data2

    // println!("{}", data1); // This line would cause a compile-time error because data1 is no longer valid
    println!("{}", data2); // data2 is now the owner of the String
} // data2 goes out of scope and the memory is freed

// if line 5 println!("{}", data1); enabled
// Error will be:
// error[E0382]: borrow of moved value: `data1`
//  --> src/language_basics/ownership_system/single_ownership.rs:5:20
//   |
// 2 |     let data1 = String::from("Hello");
//   |         ----- move occurs because `data1` has type `String`, which does not implement the `Copy` trait
// 3 |     let data2 = data1; // data1 is moved to data2
//   |                 ----- value moved here
// 4 |
// 5 |     println!("{}", data1); // This line would cause a compile-time error because data1 is no longer valid
//   |                    ^^^^^ value borrowed here after move
//   |
//   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//   |
// 3 |     let data2 = data1.clone(); // data1 is moved to data2
//   |                      ++++++++

// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `rust-lo` (bin "ownership_system_single") due to 1 previous error

// if line 5 println!("{}", data1); disabled
// Result will be:
// Hello
