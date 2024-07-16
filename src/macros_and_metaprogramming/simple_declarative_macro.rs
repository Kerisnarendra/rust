macro_rules! simple_declarative_macro {
    () => {
        println!("Hello, I am simple declarative macro!")
    };
}
fn main() {
    simple_declarative_macro!();
}

// Result will be:
// Hello, I am simple declarative macro!
