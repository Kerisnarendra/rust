fn reverse_words(statement: &str) -> String {
    let words: Vec<&str> = statement.split_whitespace().collect();

    let reversed_words: Vec<String> = words
        .iter()
        .map(|&word| word.chars().rev().collect())
        .collect();

    let reversed_statement = reversed_words.join(" ");

    reversed_statement
}

fn main() {
    let statement = "Hello World Rust Programming";
    let reversed_statement = reverse_words(statement);
    println!("Original: {}", statement);
    println!("Reversed: {}", reversed_statement);
}
