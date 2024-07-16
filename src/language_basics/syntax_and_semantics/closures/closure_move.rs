use std::ptr::addr_of;

fn main() {
    let x = 20;

    let move_closure = move || {
        println!("x address in move closure:{:?}", addr_of!(x));
        println!("x value in move closure: {}", x)
    };
    move_closure();
    println!("x address in main:{:?}", addr_of!(x));
    println!("x value in main: {}", x);
}

// Result will be:
// x address in move closure:0x7ffcd577a4ac
// x value in move closure: 20
// x address in main:0x7ffcd577a4a8
// x value in main: 20
