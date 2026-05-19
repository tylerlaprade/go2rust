use go2rust_stdlib_stubs::*;
fn main() {
    example_com_externalinit_dep::__go_init_all();

    println!("{}", format!("{}", (*example_com_externalinit_dep::is_enabled().borrow().as_ref().unwrap())));
}