/// GetYear is the canonical example from the external feedback: a function
/// returning a predeclared scalar should emit a bare Rust scalar, not a
/// wrapped Rc<RefCell<Option<i32>>>. There is no addressable callee return
/// slot to preserve, so the wrapping had no semantic effect.
pub fn get_year() -> i32 {

    return 2024 as i32;
}

fn main() {
    println!("{}", format!("{}", get_year()));
}