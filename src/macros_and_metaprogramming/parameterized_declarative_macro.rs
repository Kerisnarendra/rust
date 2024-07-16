macro_rules! parameterized_declarative_macro {
    (x => $e:expr) => {
        println!("Hi, I am parameterized declarative macro!");
        println!("My x parameter is {}", $e);
    };
    (y => $e:expr) => {
        println!("My y parameter is {}", $e);
    };
}
fn main() {
    parameterized_declarative_macro!(x => 10);
    parameterized_declarative_macro!(y => 10 + 20);
}

// Result will be:
// Hi, I am parameterized declarative macro!
// My x parameter is 10
// My y parameter is 30
