use proc_macro::return_as_is;

#[return_as_is]
fn basic_function() {}

#[return_as_is(input)]
fn function_with_input() {}

fn main() {
    basic_function();
    function_with_input();
}

// Result will be:
// attr: ""
// item: "fn basic_function() {}"
// attr: "input"
// item: "fn function_with_input() {}"
