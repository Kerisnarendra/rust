macro_rules! print_ex {
    ($e: expr) => {
        println!("{:?} = {:?}", stringify!($e), $e)
    };
}

fn main() {
    print_ex!({
        let x = 20;
        let y = 30;
        x + y + 10 * 3 * 100
    })
}
