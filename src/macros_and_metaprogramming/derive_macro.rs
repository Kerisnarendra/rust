extern crate proc_macro;
use proc_macro::AnswerFn;

#[derive(AnswerFn)]
struct Struct;

fn main() {
    println!("{}", answer());
}

// Result will be:
// 42
