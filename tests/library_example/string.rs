pub fn repeat(s: String, n: i32) -> String {

    let mut result = "".to_string();
    let mut i = 0;
    while i < n {
        result.push_str(&s);
        i += 1;
    }
    return result;
}