fn main() {
    let s1 = "Hello";
    let s2 = " World";
    let result = concatenate_strings(&s1, &s2);
    println!("{}", result);
}

fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

