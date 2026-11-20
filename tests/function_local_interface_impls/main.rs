use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


thread_local! {
    static GO_EMBEDDED_OWNER_REGISTRY: RefCell<std::collections::HashMap<usize, Box<dyn Any>>> = RefCell::new(std::collections::HashMap::new());
}

fn go_register_embedded_owner<T: 'static>(embedded_key: usize, owner: Rc<RefCell<Option<T>>>) {
    GO_EMBEDDED_OWNER_REGISTRY.with(|registry| {
        registry.borrow_mut().insert(embedded_key, Box::new(owner));
    });
}

fn go_lookup_embedded_owner<T: 'static>(embedded_key: usize, target: &str) -> Rc<RefCell<Option<T>>> {
    GO_EMBEDDED_OWNER_REGISTRY.with(|registry| {
        let registry = registry.borrow();
        let owner = registry.get(&embedded_key).unwrap_or_else(|| panic!("embedded owner registry missing {}", target));
        owner
            .downcast_ref::<Rc<RefCell<Option<T>>>>()
            .unwrap_or_else(|| panic!("embedded owner registry type mismatch for {}", target))
            .clone()
    })
}

pub trait incer: std::fmt::Display + Any {
    fn __go_clone_box_incer(&self) -> Box<dyn incer>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_incer(&self, other: &dyn incer) -> bool;
    fn inc(&mut self);
}

impl Clone for Box<dyn incer> {
    fn clone(&self) -> Self {
        incer::__go_clone_box_incer(self.as_ref())
    }
}

pub trait namer: std::fmt::Display + Any {
    fn __go_clone_box_namer(&self) -> Box<dyn namer>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_namer(&self, other: &dyn namer) -> bool;
    fn name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn namer> {
    fn clone(&self) -> Self {
        namer::__go_clone_box_namer(self.as_ref())
    }
}

pub trait speaker: std::fmt::Display + Any {
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_speaker(&self, other: &dyn speaker) -> bool;
    fn name(&self) -> Rc<RefCell<Option<String>>>;
    fn sound(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn speaker> {
    fn clone(&self) -> Self {
        speaker::__go_clone_box_speaker(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct animal {
    pub name: Rc<RefCell<Option<String>>>,
}

impl animal {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for animal {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for animal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct dog {
    pub animal: Rc<RefCell<Option<animal>>>,
    pub breed: Rc<RefCell<Option<String>>>,
}

impl dog {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.animal.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.breed.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            animal: __go_clone_0_0,
            breed: __go_clone_1_0,
        }
    }
}


impl Default for dog {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(animal::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            animal: __go_default_0_0,
            breed: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for dog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.animal.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.breed.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct cat {
    pub animal: Rc<RefCell<Option<animal>>>,
    pub lives: Rc<RefCell<Option<i64>>>,
}

impl cat {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.animal.clone();
        let __go_clone_1_0 = { let __guard = self.lives.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            animal: __go_clone_0_0,
            lives: __go_clone_1_0,
        }
    }
}


impl Default for cat {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(None));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            animal: __go_default_0_0,
            lives: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for cat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.animal.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.lives.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct counter {
    pub n: Rc<RefCell<Option<i64>>>,
}

impl counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for counter {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl animal {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl namer for animal {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        animal::name(self)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer> {
        Box::new(self.clone()) as Box<dyn namer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &dyn namer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<animal>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct animalPtr(pub Rc<RefCell<Option<animal>>>);

impl std::fmt::Display for animalPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl namer for animalPtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        animal::name(__recv)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer> {
        Box::new(self.clone()) as Box<dyn namer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &dyn namer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<animalPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl dog {
    pub fn sound(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("woof".to_string())))
    }

    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.animal.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }
}

impl namer for dog {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        dog::name(self)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer> {
        Box::new(self.clone()) as Box<dyn namer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &dyn namer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<dog>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct dogPtr(pub Rc<RefCell<Option<dog>>>);

impl std::fmt::Display for dogPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl namer for dogPtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        dog::name(__recv)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer> {
        Box::new(self.clone()) as Box<dyn namer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &dyn namer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<dogPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl speaker for dog {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        dog::name(self)
    }
    fn sound(&self) -> Rc<RefCell<Option<String>>> {
        dog::sound(self)
    }
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker> {
        Box::new(self.clone()) as Box<dyn speaker>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_speaker(&self, other: &dyn speaker) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<dog>() {
            false
        } else {
            false
        }
    }
}

impl speaker for dogPtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        dog::name(__recv)
    }
    fn sound(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        dog::sound(__recv)
    }
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker> {
        Box::new(self.clone()) as Box<dyn speaker>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_speaker(&self, other: &dyn speaker) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<dogPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl cat {
    pub fn sound(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("meow".to_string())))
    }

    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.animal.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }
}

impl namer for cat {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        cat::name(self)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer> {
        Box::new(self.clone()) as Box<dyn namer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &dyn namer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<cat>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct catPtr(pub Rc<RefCell<Option<cat>>>);

impl std::fmt::Display for catPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl namer for catPtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        cat::name(__recv)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer> {
        Box::new(self.clone()) as Box<dyn namer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &dyn namer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<catPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl speaker for cat {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        cat::name(self)
    }
    fn sound(&self) -> Rc<RefCell<Option<String>>> {
        cat::sound(self)
    }
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker> {
        Box::new(self.clone()) as Box<dyn speaker>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_speaker(&self, other: &dyn speaker) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<cat>() {
            false
        } else {
            false
        }
    }
}

impl speaker for catPtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        cat::name(__recv)
    }
    fn sound(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        cat::sound(__recv)
    }
    fn __go_clone_box_speaker(&self) -> Box<dyn speaker> {
        Box::new(self.clone()) as Box<dyn speaker>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_speaker(&self, other: &dyn speaker) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<catPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl counter {
    pub fn inc(&mut self) {
        { let __target = self.n.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

impl incer for counter {
    fn inc(&mut self) {
        counter::inc(self)
    }
    fn __go_clone_box_incer(&self) -> Box<dyn incer> {
        Box::new(self.clone()) as Box<dyn incer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_incer(&self, other: &dyn incer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<counter>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct counterPtr(pub Rc<RefCell<Option<counter>>>);

impl std::fmt::Display for counterPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl incer for counterPtr {
    fn inc(&mut self) {
        let mut __recv_guard = self.0.borrow_mut();
        let __recv = __recv_guard.as_mut().unwrap();
        counter::inc(__recv)
    }
    fn __go_clone_box_incer(&self) -> Box<dyn incer> {
        Box::new(self.clone()) as Box<dyn incer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_incer(&self, other: &dyn incer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<counterPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    
    let mut all: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn speaker>>>>>>>> = Rc::new(RefCell::new(None));
    { let new_val = { let __append_target = all.clone(); let __append_value = Rc::new(RefCell::new(Some(Box::new(dog { animal: Rc::new(RefCell::new(Some(animal { name: Rc::new(RefCell::new(Some("rex".to_string()))), ..Default::default() }))), breed: Rc::new(RefCell::new(Some("lab".to_string()))), ..Default::default() }) as Box<dyn speaker>))); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; all = new_val; };
    { let new_val = { let __append_target = all.clone(); let __append_value = Rc::new(RefCell::new(Some(Box::new(catPtr({
        let __owner = Rc::new(RefCell::new(Some(cat { animal: Rc::new(RefCell::new(Some(animal { name: Rc::new(RefCell::new(Some("tom".to_string()))), ..Default::default() }))).clone(), lives: Rc::new(RefCell::new(Some(9))), ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.borrow(); __owner_guard.as_ref().unwrap().animal.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.borrow(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    }.clone())) as Box<dyn speaker>))); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; all = new_val; };
    { let __range_holder = all.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        println!("{} {} {}", format!("{}", (*(*s.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap())), format!("{}", "says".to_string()), format!("{}", (*(*s.borrow().as_ref().unwrap()).sound().borrow().as_ref().unwrap())));
    } }

    let mut names = Rc::new(RefCell::new(Some(Vec::<String>::new())));
    { let __range_holder = all.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        {
    let _ts_subject = s.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn speaker>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<dog>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<dog>()).unwrap().clone())));
        { let new_val = { let __append_target = names.clone(); let __append_value = format!("{}{}", "dog:".to_string(), (*(*v.borrow().as_ref().unwrap()).breed.borrow().as_ref().unwrap()).clone()); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; names = new_val; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<catPtr>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<catPtr>()).unwrap().0.clone();
        { let new_val = { let __append_target = names.clone(); let __append_value = (*Rc::new(RefCell::new(Some(format!("{}{}", format!("{}", "cat:".to_string()), format!("{}", (*(*v.borrow().as_ref().unwrap()).lives.borrow().as_ref().unwrap())))))).borrow().as_ref().unwrap()).clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; names = new_val; };;
    }
    }
    } }
    { let __sort_target = names.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some({ let __parts = (*names.borrow()).as_ref().cloned().unwrap_or_default(); let __sep = ",".to_string(); __parts.join(&__sep) }))).borrow().as_ref().unwrap())));

    
    let mut c = Rc::new(RefCell::new(Some(counter { n: Rc::new(RefCell::new(Some(0))) })));
    let mut i = Rc::new(RefCell::new(Some(Box::new(counterPtr(c.clone())) as Box<dyn incer>)));
    (*i.borrow_mut().as_mut().unwrap()).inc();
    (*i.borrow_mut().as_mut().unwrap()).inc();
    println!("{} {}", format!("{}", "count".to_string()), format!("{}", (*(*c.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));

    
    let mut n = Rc::new(RefCell::new(Some(Box::new(dog { animal: Rc::new(RefCell::new(Some(animal { name: Rc::new(RefCell::new(Some("fido".to_string()))), ..Default::default() }))), breed: Rc::new(RefCell::new(Some("pug".to_string()))), ..Default::default() }) as Box<dyn namer>)));
    {
        let (mut sp, mut ok) = ({
        let val = n.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn namer>::__go_as_any(any_val.as_ref()).downcast_ref::<catPtr>() {
                (Rc::new(RefCell::new(Some(Box::new(typed_val.clone()) as Box<dyn speaker>))), true)
            } else if let Some(typed_val) = <dyn namer>::__go_as_any(any_val.as_ref()).downcast_ref::<dog>() {
                (Rc::new(RefCell::new(Some(Box::new(typed_val.clone()) as Box<dyn speaker>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Box<dyn speaker>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Box<dyn speaker>>)), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "also a speaker:".to_string()), format!("{}", (*(*sp.borrow().as_ref().unwrap()).sound().borrow().as_ref().unwrap())));;
        }
    }
    print!("{}\n", (*(*n.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap()));
}