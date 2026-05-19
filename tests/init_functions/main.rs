use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// Struct with initialization
#[derive(Debug, Clone)]
pub struct Config {
    pub name: Rc<RefCell<Option<String>>>,
    pub version: Rc<RefCell<Option<String>>>,
    pub debug: Rc<RefCell<Option<bool>>>,
}

impl Config {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, version: { let __guard = self.version.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, debug: { let __guard = self.debug.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Config {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))), version: Rc::new(RefCell::new(Some(String::new()))), debug: Rc::new(RefCell::new(Some(false))) }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.version.borrow().as_ref().unwrap()), (*self.debug.borrow().as_ref().unwrap()))
    }
}


pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static globalCounter: GoGlobal<i32> = GoGlobal::new();

pub(crate) static initialized: GoGlobal<bool> = GoGlobal::new();

pub(crate) static configData: GoGlobal<BTreeMap<String, Rc<RefCell<Option<String>>>>> = GoGlobal::new();

pub(crate) static computedValue: GoGlobal<i32> = GoGlobal::new();

pub(crate) static appConfig: GoGlobal<Config> = GoGlobal::new();


fn __go_init_globals() {
    *globalCounter.borrow_mut() = Some(0);
    *initialized.borrow_mut() = Some(false);
    *configData.borrow_mut() = Some(BTreeMap::new());
    *computedValue.borrow_mut() = Some(0);
    *appConfig.borrow_mut() = Some(Default::default());
    *computedValue.borrow_mut() = Some((*compute_initial_value().borrow().as_ref().unwrap()).clone());
}


/// First init function
fn __go_init_0() {
    println!("{}", format!("{}", "First init function called".to_string()));
    { let new_val = 10; *globalCounter.borrow_mut() = Some(new_val); };
    { let new_val = true; *initialized.borrow_mut() = Some(new_val); };
}

/// Second init function (they run in order)
fn __go_init_1() {
    println!("{}", format!("{}", "Second init function called".to_string()));
    { let mut guard = globalCounter.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 5); };

        // Initialize map
    { let new_val = { let __collection_holder = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::new()))).clone(); let __collection_guard = __collection_holder.borrow(); (*__collection_guard).clone() }; *configData.borrow_mut() = new_val; };
    { let __map_key = "version".to_string(); let __map_value = Rc::new(RefCell::new(Some("1.0".to_string()))); (*configData.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "author".to_string(); let __map_value = Rc::new(RefCell::new(Some("go2rust".to_string()))); (*configData.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
}

/// Third init function
fn __go_init_2() {
    println!("{}", format!("{}", "Third init function called".to_string()));
    if (*initialized.borrow().as_ref().unwrap()) {
        print!("Global counter initialized to: {}\n", { let __v = (*globalCounter.borrow().as_ref().unwrap()).clone(); __v });
    }

        // Add more config
    { let __map_key = "build".to_string(); let __map_value = Rc::new(RefCell::new(Some("debug".to_string()))); (*configData.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "target".to_string(); let __map_value = Rc::new(RefCell::new(Some("rust".to_string()))); (*configData.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
}

pub fn compute_initial_value() -> Rc<RefCell<Option<i32>>> {

    println!("{}", format!("{}", "Computing initial value during package initialization".to_string()));
    return Rc::new(RefCell::new(Some(42 * 2)));
}

/// Another init function that runs after variable initialization
fn __go_init_3() {
    println!("{}", format!("{}", "Fourth init function called".to_string()));
    print!("Computed value is: {}\n", { let __v = (*computedValue.borrow().as_ref().unwrap()).clone(); __v });

        // Modify the computed value
    { let mut guard = computedValue.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 10); };
}

fn __go_init_4() {
    println!("{}", format!("{}", "Fifth init function - initializing app config".to_string()));
    { let new_val = Config { name: Rc::new(RefCell::new(Some("Go2Rust Transpiler".to_string()))), version: Rc::new(RefCell::new(Some("0.1.0".to_string()))), debug: Rc::new(RefCell::new(Some(true))), ..Default::default() }; *appConfig.borrow_mut() = Some(new_val); };
}

/// Init function that might panic (for testing error handling)
fn __go_init_5() {
    println!("{}", format!("{}", "Sixth init function - with potential panic handling".to_string()));
    println!("{}", format!("{}", "Sixth init function completed successfully".to_string()));
}

/// Helper function for init
pub fn setup_logging() {
    println!("{}", format!("{}", "Setting up logging system...".to_string()));
}

fn __go_init_6() {
    println!("{}", format!("{}", "Seventh init function - setting up subsystems".to_string()));
    setup_logging();

        // Validate configuration
    if ((*configData.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) == (0 as i32) {
        println!("{}", format!("{}", "Warning: No configuration data found".to_string()));
    } else {
        print!("Configuration loaded with {} entries\n", (*configData.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0));
    }
}

fn main() {
    __go_init_all();
    println!("{}", format!("{}", "\n=== Main function started ===".to_string()));

        // Show that all init functions have run
    print!("Global counter: {}\n", { let __v = (*globalCounter.borrow().as_ref().unwrap()).clone(); __v });
    print!("Initialized flag: {}\n", { let __v = (*initialized.borrow().as_ref().unwrap()).clone(); __v });
    print!("Computed value: {}\n", { let __v = (*computedValue.borrow().as_ref().unwrap()).clone(); __v });

    println!("{}", format!("{}", "\nConfiguration data:".to_string()));
    print!("  version: {}\n", (*configData.borrow().as_ref().unwrap()).get(&"version".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()));
    print!("  author: {}\n", (*configData.borrow().as_ref().unwrap()).get(&"author".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()));
    print!("  build: {}\n", (*configData.borrow().as_ref().unwrap()).get(&"build".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()));
    print!("  target: {}\n", (*configData.borrow().as_ref().unwrap()).get(&"target".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()));

    print!("\nApp config: {{Name:{} Version:{} Debug:{}}}\n", (*(*appConfig.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone(), (*(*appConfig.borrow().as_ref().unwrap()).version.borrow().as_ref().unwrap()).clone(), (*(*appConfig.borrow().as_ref().unwrap()).debug.borrow().as_ref().unwrap()));

        // Demonstrate that init functions only run once
    println!("{}", format!("{}", "\n=== Calling functions that were used in init ===".to_string()));
    print!("Calling computeInitialValue() again: {}\n", (*compute_initial_value().borrow().as_ref().unwrap()));
    setup_logging();

        // Show that package variables retain their init values
    print!("Global counter still: {}\n", { let __v = (*globalCounter.borrow().as_ref().unwrap()).clone(); __v });

        // Modify package variables
    { let new_val = 100; *globalCounter.borrow_mut() = Some(new_val); };
    print!("Modified global counter: {}\n", { let __v = (*globalCounter.borrow().as_ref().unwrap()).clone(); __v });

    println!("{}", format!("{}", "\n=== Demonstrating init execution order ===".to_string()));
    println!("{}", format!("{}", "1. Package-level variable declarations".to_string()));
    println!("{}", format!("{}", "2. Package-level variable initializations (like computedValue)".to_string()));
    println!("{}", format!("{}", "3. Init functions in the order they appear in source".to_string()));
    println!("{}", format!("{}", "4. Main function".to_string()));

    println!("{}", format!("{}", "\n=== Main function completed ===".to_string()));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
    self::__go_init_1();
    self::__go_init_2();
    self::__go_init_3();
    self::__go_init_4();
    self::__go_init_5();
    self::__go_init_6();
}
