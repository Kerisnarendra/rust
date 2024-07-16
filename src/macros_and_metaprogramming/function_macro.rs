macro_rules! build_function_macro {
    ($function_name:ident) => {
        fn $function_name() {
            println!("Hello, I am function macro!");
            println!("and you called me {:?}()", stringify!($function_name));
        }
    };
}
fn main() {
    build_function_macro!(function_name);
    function_name();
}
