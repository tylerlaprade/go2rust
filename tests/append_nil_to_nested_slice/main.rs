use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut stack: Rc<RefCell<Option<Vec<Vec<i64>>>>> = Rc::new(RefCell::new(None));
    { let new_val = { let __append_target = stack.clone(); let __append_value = Default::default(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; stack = new_val; };
    { let new_val = { let __append_target = stack.clone(); let __append_value = (*Rc::new(RefCell::new(Some(vec![1i64, 2i64]))).borrow().as_ref().unwrap()).clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; stack = new_val; };
    let mut total = Rc::new(RefCell::new(Some(0i64)));
    { let __range_holder = stack.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for row in __range_values.iter() {
        for v in row.iter().copied() {
        { let __rhs = v; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    } }
    {
            let __go_print_arg_0 = format!("{}", (*stack.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0));
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", { let __v = (*total.borrow().as_ref().unwrap()).clone(); __v });
            eprintln!("{}", __go_print_arg_0)
        };
}