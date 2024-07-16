use std::ptr::addr_of;

fn main() {
    let x = 20;

    let fn_closure = || {
        println!("x address in fn closure:{:?}", addr_of!(x));
        println!("x value in fn closure: {}", x);
        return x;
    };
    println!("x address in fn closure:{:?}", addr_of!(x));
    println!("result of main closure: {}", fn_closure());
}

// Result will be:
// x address in fn closure:0x7ffd4446dadc
// x address in fn closure:0x7ffd4446dadc
// x value in fn closure: 20
// result of main closure: 20
