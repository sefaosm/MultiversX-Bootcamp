// bootcamp-tasks/src/bin/concatenate-strings.rs
// # Concatenate Strings
// This program concatenates two strings and prints the result.
//
// Author: Sefa Osmanoğlu

fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut result = String::new();
    result.push_str(str1);
    result.push_str(str2);
    result
}

fn main() {
    let str1 = String::from("Hello,");
    let str2 = String::from(" World!");

    let concatenated_string = concatenate_strings(&str1, &str2);

    println!("Concatenated string: {}", concatenated_string);
}