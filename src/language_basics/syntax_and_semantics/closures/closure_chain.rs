fn main() {
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // Filter the months which ended with er
    let filtered_months = months
        .into_iter()
        .filter(|month| month.contains("er"))
        .collect::<Vec<&str>>();

    println!("{:?}", filtered_months);
}

// Result will be:
// ["September", "October", "November", "December"]
