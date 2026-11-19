use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn matches_current(value: Rc<RefCell<Option<Box<dyn example_com_importedpointer_dep::r#mod::Reader>>>>) -> bool {
    { let __left_holder = value.clone(); let __left_guard = __left_holder.borrow(); let __left_opt: Option<&dyn example_com_importedpointer_dep::r#mod::Reader> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = example_com_importedpointer_dep::r#mod::NodePtr(example_com_importedpointer_dep::__go_get_global_Current()); let __right_opt: Option<&dyn example_com_importedpointer_dep::r#mod::Reader> = Some(&__right_wrapper as &dyn example_com_importedpointer_dep::r#mod::Reader); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_reader(__right), _ => false }; __eq }
}

fn main() {
    ::example_com_importedpointer_dep::__go_init_all();

    let mut original = example_com_importedpointer_dep::__go_get_global_Current();
    let mut view = example_com_importedpointer_dep::view();
    println!("{} {}", format!("{}", { let __left = original.clone(); let __right = example_com_importedpointer_dep::__go_get_global_Current(); let __both_nil = (*__left.borrow()).is_none() && (*__right.borrow()).is_none(); let __eq = __both_nil || Rc::ptr_eq(&__left, &__right); __eq }), format!("{}", matches_current(view.clone())));
    example_com_importedpointer_dep::__go_clear_global_Current();
    let mut cleared = example_com_importedpointer_dep::get();
    println!("{} {} {}", format!("{}", { let __nil_target = example_com_importedpointer_dep::__go_get_global_Current(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }), format!("{}", { let __nil_result = (*cleared.borrow()).is_none(); __nil_result }), format!("{}", (*(*original.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));
    let mut other = example_com_importedpointer_dep::__go_get_global_Other();
    let mut count = Rc::new(RefCell::new(Some(0)));
    {
        let __tmp_0 = example_com_importedpointer_dep::__go_clear_global_Other;
        let __tmp_1 = 1;
        __tmp_0();
        *count.borrow_mut() = Some(__tmp_1);
    };
    println!("{} {} {}", format!("{}", { let __nil_target = example_com_importedpointer_dep::__go_get_global_Other(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }), format!("{}", (*(*other.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())), format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }));
}