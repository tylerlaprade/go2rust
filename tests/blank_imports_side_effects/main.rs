pub fn __go_init_0() {
    println!("{}", "main package init".to_string());
}

fn main() {
    __go_init_all();
    println!("{}", "main function".to_string());
}

pub(crate) fn __go_init_all() {
    __go_init_0();
}
