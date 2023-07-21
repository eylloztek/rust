fn main() {
    let string_first = String::from("Hello");
    let string_second = String::from(" world");
    let concatenated_string = concatenate_strings(&string_first, &string_second);
    println!("{}",concatenated_string);
    
}

fn concatenate_strings(s: &String, t: &String) -> String{
    let mut result = String::from("");
    result.push_str(&s);
    result.push_str(&t);
    result
}