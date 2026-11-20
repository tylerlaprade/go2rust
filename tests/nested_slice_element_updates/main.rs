use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.borrow();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.borrow();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}
fn format_nested_slice<T, C, Inner>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        format_nested_slice_values(s.as_ref())
    } else {
        "[]".to_string()
    }
}
fn format_nested_slice_values<T, Inner>(slice: &[Inner]) -> String
where
    Inner: AsRef<[T]>,
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|inner| format_slice_values(inner.as_ref())).collect();
    format!("[{}]", formatted.join(" "))
}

#[derive(Debug, Clone, Default)]
pub struct Grid(pub Rc<RefCell<Option<Vec<Vec<i64>>>>>);


#[derive(Debug, Clone, Default)]
pub struct Sheet {
    pub cells: Rc<RefCell<Option<Vec<Vec<f64>>>>>,
    pub names: Rc<RefCell<Option<Vec<String>>>>,
}

impl Sheet {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.cells.clone();
        let __go_clone_1_0 = self.names.clone();
        Self {
            cells: __go_clone_0_0,
            names: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for Sheet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_nested_slice(&self.cells));
        let __go_fmt_1 = format!("{}", format_slice(&self.names));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Sheet {
    pub fn bump(&mut self, i: Rc<RefCell<Option<i64>>>) {
        { let __outer_idx = ((*i.borrow().as_ref().unwrap())) as usize; let __idx = ((*i.borrow().as_ref().unwrap())) as usize; let __rhs = 0.5; let __seq_holder = self.cells.clone(); let mut __seq_guard = __seq_holder.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] + __rhs; };
        { let __idx = ((*i.borrow().as_ref().unwrap())) as usize; let __rhs = "!".to_string(); let __seq_holder = self.names.clone(); let mut __seq_guard = __seq_holder.borrow_mut(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() + &__rhs; };
    }
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(vec![vec![1i64, 2i64], vec![3i64, 4i64]])));
    { let __outer_idx = (0) as usize; let __idx = (1) as usize; let __rhs = 10; let mut __seq_guard = a.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] + __rhs; };
    {
        let __go_lhs_sequence_0 = a.clone();
        let __go_lhs_outer_index_0_0 = (1) as usize;
        let __go_lhs_index_0 = (0) as usize;
        let __go_lhs_sequence_1 = a.clone();
        let __go_lhs_outer_index_1_0 = (1) as usize;
        let __go_lhs_index_1 = (1) as usize;
        let __tmp_0 = (*a.borrow().as_ref().unwrap())[(1) as usize].clone()[(1) as usize].clone();
        let __tmp_1 = (*a.borrow().as_ref().unwrap())[(1) as usize].clone()[(0) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_outer_index_0_0][__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_outer_index_1_0][__go_lhs_index_1] = __tmp_1;
    };
    { let __outer_idx = (0) as usize; let __idx = (0) as usize; let mut __seq_guard = a.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] + 1; }
    { let __outer_idx = (1) as usize; let __idx = (1) as usize; let __rhs = 2; let mut __seq_guard = a.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] - __rhs; };
    let mut g = Rc::new(RefCell::new(Some(Grid(Rc::new(RefCell::new(Some(vec![vec![1i64, 2i64], vec![3i64, 4i64]])))))));
    { let __outer_idx = (0) as usize; let __idx = (1) as usize; let __rhs = 7; let __seq_holder = { let __named_slice = (*g.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let mut __seq_guard = __seq_holder.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] * __rhs; };
    {
        let __go_lhs_sequence_0 = { let __named_slice = (*g.borrow().as_ref().unwrap()).0.clone(); __named_slice }.clone();
        let __go_lhs_outer_index_0_0 = (1) as usize;
        let __go_lhs_index_0 = (0) as usize;
        let __go_lhs_sequence_1 = { let __named_slice = (*g.borrow().as_ref().unwrap()).0.clone(); __named_slice }.clone();
        let __go_lhs_outer_index_1_0 = (1) as usize;
        let __go_lhs_index_1 = (1) as usize;
        let __tmp_0 = { let __seq_holder = { let __named_slice = (*g.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }[(1) as usize].clone();
        let __tmp_1 = { let __seq_holder = { let __named_slice = (*g.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }[(0) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_outer_index_0_0][__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_outer_index_1_0][__go_lhs_index_1] = __tmp_1;
    };
    { let __outer_idx = (0) as usize; let __idx = (0) as usize; let __seq_holder = { let __named_slice = (*g.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let mut __seq_guard = __seq_holder.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] - 1; }
    println!("{} {}", format!("{}", format_nested_slice(&a)), format!("{}", format_nested_slice(&{ let __named_slice = (*g.borrow().as_ref().unwrap()).0.clone(); __named_slice })));

    let mut cube = Rc::new(RefCell::new(Some(vec![vec![vec![1i64, 2i64], vec![3i64]], vec![vec![4i64]]])));
    { let __outer_idx = (0) as usize; let __outer_idx_1 = (1) as usize; let __idx = (0) as usize; let __rhs = 30; (*cube.borrow_mut().as_mut().unwrap())[__outer_idx][__outer_idx_1][__idx] = __rhs; };
    { let __outer_idx = (1) as usize; let __outer_idx_1 = (0) as usize; let __idx = (0) as usize; let __rhs = 5; let mut __seq_guard = cube.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx][__outer_idx_1]; __seq[__idx] = __seq[__idx] * __rhs; };
    { let __outer_idx = (0) as usize; let __outer_idx_1 = (0) as usize; let __idx = (1) as usize; let mut __seq_guard = cube.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx][__outer_idx_1]; __seq[__idx] = __seq[__idx] - 1; }
    println!("{} {}", format!("{}", { let __formatted = format_nested_slice_values(&(*cube.borrow().as_ref().unwrap())[(0) as usize].clone()); __formatted }), format!("{}", (*cube.borrow().as_ref().unwrap())[(1) as usize].clone()[(0) as usize].clone()[(0) as usize].clone()));

    let mut board: Rc<RefCell<Option<[[String; 3]; 3]>>> = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 3, _>(|_| std::array::from_fn::<_, 3, _>(|_| String::new())))));
    let mut i = Rc::new(RefCell::new(Some(0i64)));
    while (*i.borrow().as_ref().unwrap()) < 3 {
        let mut j = Rc::new(RefCell::new(Some(0i64)));
    while (*j.borrow().as_ref().unwrap()) < 3 {
        { let __outer_idx = ((*i.borrow().as_ref().unwrap())) as usize; let __idx = ((*j.borrow().as_ref().unwrap())) as usize; let __rhs = ".".to_string(); (*board.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __outer_idx = (1) as usize; let __idx = (1) as usize; let __rhs = "X".to_string(); (*board.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    { let __outer_idx = (0) as usize; let __idx = (2) as usize; let __rhs = "O".to_string(); let mut __seq_guard = board.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx].clone() + &__rhs; };
    println!("{}", format!("{}", format_nested_slice(&board)));

    let mut grid = Rc::new(RefCell::new(Some([std::array::from_fn::<_, 3, _>(|_| 0), std::array::from_fn::<_, 3, _>(|_| 0), std::array::from_fn::<_, 3, _>(|_| 0)])));
    let mut i = Rc::new(RefCell::new(Some(0i64)));
    while (*i.borrow().as_ref().unwrap()) < 3 {
        { let __outer_idx = ((*i.borrow().as_ref().unwrap())) as usize; let __idx = ((*i.borrow().as_ref().unwrap())) as usize; let __rhs = (*i.borrow().as_ref().unwrap()) + 1; (*grid.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
        { let __outer_idx = ((*i.borrow().as_ref().unwrap())) as usize; let __idx = (2 - (*i.borrow().as_ref().unwrap())) as usize; let __rhs = 10; let mut __seq_guard = grid.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] + __rhs; };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", format!("{}", format_nested_slice(&grid)));

    let mut paths = Rc::new(RefCell::new(Some(vec![vec![]; (4) as usize])));
    for i in 0..(({ let __range_holder = paths.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = (i) as usize; let __rhs = (*Rc::new(RefCell::new(Some(vec![0; (4) as usize]))).borrow().as_ref().unwrap()).clone(); (*paths.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
        { let __outer_idx = (i) as usize; let __idx = (0) as usize; let __rhs = 1; (*paths.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    }
    let mut i = Rc::new(RefCell::new(Some(1i64)));
    while (*i.borrow().as_ref().unwrap()) < 4 {
        { let __outer_idx = (0) as usize; let __idx = ((*i.borrow().as_ref().unwrap())) as usize; let __rhs = 1; (*paths.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
        let mut j = Rc::new(RefCell::new(Some(1i64)));
    while (*j.borrow().as_ref().unwrap()) < 4 {
        { let __outer_idx = ((*i.borrow().as_ref().unwrap())) as usize; let __idx = ((*j.borrow().as_ref().unwrap())) as usize; let __rhs = (*paths.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone()[((*j.borrow().as_ref().unwrap())) as usize].clone() + (*paths.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); (*paths.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {}", format!("{}", (*paths.borrow().as_ref().unwrap())[(3) as usize].clone()[(3) as usize].clone()), format!("{}", format_nested_slice(&paths)));

    let mut s = Rc::new(RefCell::new(Some(Sheet { cells: Rc::new(RefCell::new(Some(vec![vec![1.0, 2.0], vec![3.0, 4.0]]))), names: Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()]))), ..Default::default() })));
    (*s.borrow_mut().as_mut().unwrap()).bump(Rc::new(RefCell::new(Some(0))));
    (*s.borrow_mut().as_mut().unwrap()).bump(Rc::new(RefCell::new(Some(1))));
    {
        let __go_lhs_sequence_0 = (*s.borrow().as_ref().unwrap()).cells.clone();
        let __go_lhs_outer_index_0_0 = (1) as usize;
        let __go_lhs_index_0 = (0) as usize;
        let __go_lhs_sequence_1 = (*s.borrow().as_ref().unwrap()).cells.clone();
        let __go_lhs_outer_index_1_0 = (1) as usize;
        let __go_lhs_index_1 = (1) as usize;
        let __tmp_0 = (*(*s.borrow().as_ref().unwrap()).cells.borrow().as_ref().unwrap())[(1) as usize].clone()[(1) as usize].clone();
        let __tmp_1 = (*(*s.borrow().as_ref().unwrap()).cells.borrow().as_ref().unwrap())[(1) as usize].clone()[(0) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_outer_index_0_0][__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_outer_index_1_0][__go_lhs_index_1] = __tmp_1;
    };
    let mut words = Rc::new(RefCell::new(Some(vec!["x".to_string(), "y".to_string()])));
    { let __idx = (0) as usize; let __rhs = "z".to_string(); let mut __seq_guard = words.borrow_mut(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() + &__rhs; };
    { let __idx = (1) as usize; let __rhs = (*words.borrow().as_ref().unwrap())[(0) as usize].clone(); let mut __seq_guard = words.borrow_mut(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() + &__rhs; };
    let mut tags = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::from([("k".to_string(), Rc::new(RefCell::new(Some("v".to_string()))))]))));
    { let __map_holder = tags.clone(); let __key = "k".to_string(); let __rhs = "w".to_string(); let mut __map_guard = __map_holder.borrow_mut(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Rc::new(RefCell::new(Some(String::new())))); let mut __value = __entry.borrow_mut(); * __value = Some((*__value.as_ref().unwrap()).clone() + &__rhs); };
    let mut score = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<f64>>>>::from([]))));
    { let __map_holder = score.clone(); let __key = "a".to_string(); let mut __map_guard = __map_holder.borrow_mut(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Rc::new(RefCell::new(Some(0.0)))); let mut __value = __entry.borrow_mut(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1.0); }
    { let __map_holder = score.clone(); let __key = "a".to_string(); let __rhs = 1.5; let mut __map_guard = __map_holder.borrow_mut(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Rc::new(RefCell::new(Some(0.0)))); let mut __value = __entry.borrow_mut(); * __value = Some((*__value.as_ref().unwrap()).clone() + __rhs); };
    println!("{} {} {} {} {}", format!("{}", { let __formatted = format_nested_slice_values(&(*(*s.borrow().as_ref().unwrap()).cells.borrow().as_ref().unwrap()).clone()); __formatted }), format!("{}", format_slice_values(&(*(*s.borrow().as_ref().unwrap()).names.borrow().as_ref().unwrap()).clone())), format!("{}", format_slice(&words)), format!("{}", format_map(&tags)), format!("{}", format_map(&score)));
}