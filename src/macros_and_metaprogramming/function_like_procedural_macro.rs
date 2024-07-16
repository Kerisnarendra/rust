extern crate proc_macro;
use proc_macro::make_answer;

make_answer!();

fn main() {
    println!("{}", answer());
}
