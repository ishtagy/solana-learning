fn main() {
    let string1 = String::from("Hello ");
    let string2 = String::from("world");

    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("Result: {}", concatenated_string);
}

fn concatenate_strings(str1: &String, str2: &String) -> String {
    format!("{}{}", str1, str2)
}
