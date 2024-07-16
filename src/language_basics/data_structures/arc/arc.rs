use std::sync::Arc;

fn main() {
    //  Create an arc
    let value = Arc::new(10);

    //  Clone an arc
    let cloned_value = value.clone();

    //  Access the values
    println!("Value is: {}", value);
    println!("Cloned value is: {}", cloned_value);
}
