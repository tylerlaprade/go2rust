use go2rust_stdlib_stubs::*;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


#[derive(Clone)]
struct GoSliceElemPtr<T: Clone> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
}

struct GoSliceElemRef<T: Clone> {
    value: Option<T>,
}

struct GoSliceElemMutRef<T: Clone> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
    value: Option<T>,
}

impl<T: Clone> GoSliceElemPtr<T> {
    fn new(slice: Arc<Mutex<Option<Vec<T>>>>, index: usize) -> Self {
        GoSliceElemPtr { slice, index }
    }

    fn borrow(&self) -> GoSliceElemRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    fn borrow_mut(&self) -> GoSliceElemMutRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemMutRef {
            slice: self.slice.clone(),
            index: self.index,
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemMutRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::DerefMut for GoSliceElemMutRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone> Drop for GoSliceElemMutRef<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.clone() {
            if let Some(values) = self.slice.lock().unwrap().as_mut() {
                values[self.index] = value;
            }
        }
    }
}

#[derive(Clone)]
pub struct Reader {
    pub decoder: Arc<Mutex<Option<example_com_importedembed_base::Decoder>>>,
    pub name: Arc<Mutex<Option<String>>>,
}

impl Reader {
    pub fn __go_value_clone(&self) -> Self {
        Self { decoder: { let __guard = self.decoder.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Reader {
    fn default() -> Self {
        Self { decoder: Arc::new(Mutex::new(Some(Default::default()))), name: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.decoder.lock().unwrap().as_ref().unwrap()), (*self.name.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Reader {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct pkgReader {
    pub pkg_decoder: Arc<Mutex<Option<example_com_importedembed_base::PkgDecoder>>>,
}

impl pkgReader {
    pub fn __go_value_clone(&self) -> Self {
        Self { pkg_decoder: { let __guard = self.pkg_decoder.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for pkgReader {
    fn default() -> Self {
        Self { pkg_decoder: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for pkgReader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.pkg_decoder.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for pkgReader {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl pkgReader {
    pub fn new_reader(&self, delta: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Reader>>> {
        Arc::new(Mutex::new(Some(Reader { decoder: self.new_decoder(Arc::new(Mutex::new(Some((*delta.lock().unwrap().as_ref().unwrap()).clone())))), name: Arc::new(Mutex::new(Some("frompkg".to_string()))), ..Default::default() })))
    }

    pub fn retire_reader(&self, r: Arc<Mutex<Option<Reader>>>) {
        self.retire_decoder((*r.lock().unwrap().as_ref().unwrap()).decoder.clone());
    }

    pub fn new_decoder(&self, _arg0: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<example_com_importedembed_base::Decoder>>> {
        let embedded = self.pkg_decoder.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.new_decoder(_arg0)
    }

    pub fn retire_decoder(&self, _arg0: Arc<Mutex<Option<example_com_importedembed_base::Decoder>>>) {
        let embedded = self.pkg_decoder.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.retire_decoder(_arg0)
    }
}

impl Reader {
    pub fn add(&self, _arg0: Arc<Mutex<Option<i32>>>) {
        let embedded = self.decoder.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.add(_arg0)
    }

    pub fn clone(&self) -> Arc<Mutex<Option<example_com_importedembed_base::Decoder>>> {
        let embedded = self.decoder.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.clone()
    }

    pub fn label(&self, _arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        let embedded = self.decoder.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.label(_arg0)
    }

    pub fn snapshot(&self) -> i32 {
        let embedded = self.decoder.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.snapshot()
    }
}

pub fn force_concurrent_wrappers() {
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();
}

pub fn pick_name(names: Arc<Mutex<Option<Vec<String>>>>, idx: Arc<Mutex<Option<example_com_importedembed_base::Index>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = names.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() })))
}

pub fn touch_name_ptr(names: Arc<Mutex<Option<Vec<String>>>>, idx: Arc<Mutex<Option<example_com_importedembed_base::Index>>>) {
    let _ = GoSliceElemPtr::new(names.clone(), (*{ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize);
}

fn main() {
    example_com_importedembed_base::__go_init_all();

    force_concurrent_wrappers();
    let mut r = Arc::new(Mutex::new(Some(Reader { decoder: Arc::new(Mutex::new(Some(example_com_importedembed_base::Decoder { value: Arc::new(Mutex::new(Some(3))), ..Default::default() }))), name: Arc::new(Mutex::new(Some("reader".to_string()))), ..Default::default() })));
    (*r.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(4))));
    println!("{}", format!("{}", (*(*r.lock().unwrap().as_ref().unwrap()).label(Arc::new(Mutex::new(Some("reader".to_string())))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*r.lock().unwrap().as_ref().unwrap()).snapshot()));
    let mut copied = Arc::new(Mutex::new(Some(Reader { decoder: (*r.lock().unwrap().as_ref().unwrap()).clone(), name: Arc::new(Mutex::new(Some("copy".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", (*(*copied.lock().unwrap().as_ref().unwrap()).label(Arc::new(Mutex::new(Some("copy".to_string())))).lock().unwrap().as_ref().unwrap())));
    let mut pr = Arc::new(Mutex::new(Some(pkgReader { pkg_decoder: Arc::new(Mutex::new(Some(example_com_importedembed_base::PkgDecoder { base: Arc::new(Mutex::new(Some(10))), ..Default::default() }))), ..Default::default() })));
    let mut fromPkg = (*pr.lock().unwrap().as_ref().unwrap()).new_reader(Arc::new(Mutex::new(Some(5))));
    println!("{}", format!("{}", (*{ let __recv = fromPkg.clone(); let __recv_ptr: *const Reader = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Reader }; let __result = unsafe { &*__recv_ptr }.label(Arc::new(Mutex::new(Some("frompkg".to_string())))); __result }.lock().unwrap().as_ref().unwrap())));
    (*pr.lock().unwrap().as_ref().unwrap()).retire_reader(fromPkg.clone());
    println!("{}", format!("{}", (*{ let __recv = fromPkg.clone(); let __recv_ptr: *const Reader = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Reader }; let __result = unsafe { &*__recv_ptr }.label(Arc::new(Mutex::new(Some("retired".to_string())))); __result }.lock().unwrap().as_ref().unwrap())));
    let mut idx = Arc::new(Mutex::new(Some(example_com_importedembed_base::Index(Arc::new(Mutex::new(Some(1 as i32)))))));
    println!("{}", format!("{}", (*pick_name(Arc::new(Mutex::new(Some(vec!["zero".to_string(), "one".to_string(), "two".to_string()]))), Arc::new(Mutex::new(Some((*idx.lock().unwrap().as_ref().unwrap()).clone())))).lock().unwrap().as_ref().unwrap())));
    touch_name_ptr(Arc::new(Mutex::new(Some(vec!["zero".to_string(), "one".to_string(), "two".to_string()]))), Arc::new(Mutex::new(Some((*idx.lock().unwrap().as_ref().unwrap()).clone()))));
}