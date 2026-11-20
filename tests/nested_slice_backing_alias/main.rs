use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut grid = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(vec![1, 2]))), Rc::new(RefCell::new(Some(vec![3, 4])))])));
    let mut row = (*grid.borrow().as_ref().unwrap())[(0) as usize].clone();
    { let __idx = (1) as usize; let __rhs = 9; (*row.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    {
            let __go_print_arg_0 = format!("{}", (*(*grid.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap())[(1) as usize].clone());
            eprintln!("{}", __go_print_arg_0)
        };

    let mut r = Rc::new(RefCell::new(Some(vec![5])));
    let mut rows = Rc::new(RefCell::new(Some(Vec::<Rc<RefCell<Option<Vec<i32>>>>>::new())));
    { let new_val = { let __append_target = rows.clone(); let __append_value = r.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; rows = new_val; };
    { let __idx = (0) as usize; let __rhs = 6; (*r.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    {
            let __go_print_arg_0 = format!("{}", (*(*rows.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap())[(0) as usize].clone());
            eprintln!("{}", __go_print_arg_0)
        };

    { let __range_holder = grid.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for row in __range_values.iter() {
        { let __idx = (0) as usize; let __rhs = 0; (*row.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    } }
    {
            let __go_print_arg_0 = format!("{}", (*(*grid.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap())[(0) as usize].clone());
            let __go_print_arg_1 = format!("{}", (*(*grid.borrow().as_ref().unwrap())[(1) as usize].clone().borrow().as_ref().unwrap())[(0) as usize].clone());
            eprintln!("{} {}", __go_print_arg_0, __go_print_arg_1)
        };

    let mut a = Rc::new(RefCell::new(Some([Rc::new(RefCell::new(Some(vec![7])))])));
    let mut b = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let __outer_holder = b.clone(); let __outer_idx = (0) as usize; let __idx = (0) as usize; let __rhs = 24; let __outer_guard = __outer_holder.borrow(); let __nested_holder_0 = __outer_guard.as_ref().unwrap()[__outer_idx].clone(); drop(__outer_guard); let mut __seq_guard = __nested_holder_0.borrow_mut(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __rhs; };
    {
            let __go_print_arg_0 = format!("{}", (*(*a.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap())[(0) as usize].clone());
            let __go_print_arg_1 = format!("{}", (*(*b.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap())[(0) as usize].clone());
            eprintln!("{} {}", __go_print_arg_0, __go_print_arg_1)
        };

    { let __outer_holder = grid.clone(); let __outer_idx = (1) as usize; let __idx = (1) as usize; let __rhs = 5; let __outer_guard = __outer_holder.borrow(); let __nested_holder_0 = __outer_guard.as_ref().unwrap()[__outer_idx].clone(); drop(__outer_guard); let mut __seq_guard = __nested_holder_0.borrow_mut(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + __rhs; };
    { let __append_slice = (*grid.borrow().as_ref().unwrap())[(1) as usize].clone(); let __append_value = 10; (*__append_slice.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_slice.clone() };
    {
            let __go_print_arg_0 = format!("{}", (*(*grid.borrow().as_ref().unwrap())[(1) as usize].clone().borrow().as_ref().unwrap())[(1) as usize].clone());
            let __go_print_arg_1 = format!("{}", (*(*grid.borrow().as_ref().unwrap())[(1) as usize].clone().borrow().as_ref().unwrap())[(2) as usize].clone());
            let __go_print_arg_2 = format!("{}", (*(*grid.borrow().as_ref().unwrap())[(1) as usize].clone().borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0));
            eprintln!("{} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
}