fn main() {
    let s1 = "Hello";
    let s2 = " World";

    let slice_s1 = &s1[0..3];
    let slice_s2 = &s2[0..3];

    let result = concatenate_strings(slice_s1, slice_s2);
    
    println!("{}", result);
}

fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

