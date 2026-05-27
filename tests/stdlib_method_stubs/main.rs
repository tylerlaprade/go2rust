use std::cell::{RefCell};
use std::rc::{Rc};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Alias;

impl std::fmt::Display for types_Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Alias>")
    }
}


impl types_Alias {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn rhs(&self) -> Rc<RefCell<Option<types_Type>>> {
        panic!("types_Alias.rhs bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Clone)]
pub struct types_Type {
    pub __go_id: usize,
    pub __go_value: Rc<dyn std::any::Any>,
}

impl types_Type {
    pub fn __go_from<T: 'static>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Rc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for types_Type {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Rc::new(()) }
    }
}

impl std::fmt::Debug for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}

impl std::fmt::Display for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}

impl PartialEq for types_Type {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for types_Type {}

impl PartialOrd for types_Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for types_Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


fn main() {
    let mut alias: Rc<RefCell<Option<types_Alias>>> = Rc::new(RefCell::new(None));
    if false {
        println!("{}", format!("{}", format!("{}", (*((*alias.borrow_mut().as_mut().unwrap()).rhs()).borrow().as_ref().unwrap()))));
    }
    if false {
        let (mut withRhs, mut ok) = ({
        let __asserted = alias.clone();
        (__asserted.clone(), Rc::new(RefCell::new(Some(true))))
    });
        if (*ok.borrow().as_ref().unwrap()) {
        println!("{}", format!("{}", format!("{}", (*((*withRhs.borrow().as_ref().unwrap()).rhs()).borrow().as_ref().unwrap()))));
    }
    }
    println!("{}", format!("{}", "ok".to_string()));
}