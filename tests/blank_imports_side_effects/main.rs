fn __go_init_0() {
    println!("{}", format!("{}", "main package init".to_string()));
}

fn main() {
    __go_init_all();
    println!("{}", format!("{}", "main function".to_string()));
}

pub(crate) fn __go_init_all() {
    self::__go_init_0();
}
