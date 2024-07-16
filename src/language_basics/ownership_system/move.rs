fn main() {
    let data1 = 100;
    let data2 = data1; // data1 is moved to data2

    // After the move, data1 is no longer valie

    println!("{}", data1); // This line would cause a compile-time error because data1 is no longer valid
    println!("{}", data2); // data2 is now the owner of the String
} // data2 goes out of scope and the memory is freed
