use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_any_clone, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::position::*;
use crate::r#mod::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct serializedFile {
    pub name: Arc<Mutex<Option<String>>>,
    pub base: Arc<Mutex<Option<i32>>>,
    pub size: Arc<Mutex<Option<i32>>>,
    pub lines: Arc<Mutex<Option<Vec<i32>>>>,
    pub infos: Arc<Mutex<Option<Vec<lineInfo>>>>,
}

impl serializedFile {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, base: { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, size: { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lines: self.lines.clone(), infos: self.infos.clone() }
    }
}


impl Default for serializedFile {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), base: Arc::new(Mutex::new(Some(0))), size: Arc::new(Mutex::new(Some(0))), lines: Arc::new(Mutex::new(None)), infos: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for serializedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.base.lock().unwrap().as_ref().unwrap()), (*self.size.lock().unwrap().as_ref().unwrap()), format_slice(&self.lines), format_slice(&self.infos))
    }
}

impl GoJsonDecode for serializedFile {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Name") {
            out.name = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Base") {
            out.base = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Size") {
            out.size = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Lines") {
            out.lines = <Arc<Mutex<Option<Vec<i32>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct serializedFileSet {
    pub base: Arc<Mutex<Option<i32>>>,
    pub files: Arc<Mutex<Option<Vec<serializedFile>>>>,
}

impl serializedFileSet {
    pub fn __go_value_clone(&self) -> Self {
        Self { base: { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, files: self.files.clone() }
    }
}


impl Default for serializedFileSet {
    fn default() -> Self {
        Self { base: Arc::new(Mutex::new(Some(0))), files: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for serializedFileSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.base.lock().unwrap().as_ref().unwrap()), format_slice(&self.files))
    }
}

impl GoJsonDecode for serializedFileSet {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Base") {
            out.base = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


impl crate::position::FileSet {
    /// Read calls decode to deserialize a file set into s; s must not be nil.
    pub fn read(&mut self, decode: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut ss: Arc<Mutex<Option<serializedFileSet>>> = Arc::new(Mutex::new(Some(Default::default())));
        {
        let mut err = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = decode.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(ss.clone().clone()) as Box<dyn Any + Send + Sync>)))) };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return err.clone();;
        }
    }
        self.mutex.lock();
        { let new_val = { let __selector_holder = (*ss.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.base.lock().unwrap() = Some(new_val); };
        let mut files: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::position::File>>>>>>> = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); (({ let __len_target = { let __field = (*ss.lock().unwrap().as_ref().unwrap()).files.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize])));
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = (*ss.lock().unwrap().as_ref().unwrap()).files.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        let mut f: Option<GoSliceElemPtr<serializedFile>> = Some(GoSliceElemPtr::new((*ss.lock().unwrap().as_ref().unwrap()).files.clone(), ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
        (*files.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = Arc::new(Mutex::new(Some(File { name: Arc::new(Mutex::new(Some({ let __selector_holder = (*f.as_ref().unwrap().borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), base: Arc::new(Mutex::new(Some({ let __selector_holder = (*f.as_ref().unwrap().borrow().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), size: Arc::new(Mutex::new(Some({ let __selector_holder = (*f.as_ref().unwrap().borrow().as_ref().unwrap()).size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), lines: { let __field = (*f.as_ref().unwrap().borrow().as_ref().unwrap()).lines.clone(); __field }, infos: { let __field = (*f.as_ref().unwrap().borrow().as_ref().unwrap()).infos.clone(); __field }, ..Default::default() })));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let new_val = files.clone(); self.files = new_val; };
        (*self.last.lock().unwrap().as_mut().unwrap()).store(sync_atomic::GoPtr::nil());
        self.mutex.unlock();
        return Arc::new(Mutex::new(None));
    }

    /// Write calls encode to serialize the file set s.
    pub fn write(&self, encode: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut ss: Arc<Mutex<Option<serializedFileSet>>> = Arc::new(Mutex::new(Some(Default::default())));
        self.mutex.lock();
        { let new_val = { let __selector_holder = self.base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*ss.lock().unwrap().as_ref().unwrap()).base.lock().unwrap() = Some(new_val); };
        let mut files: Arc<Mutex<Option<Vec<serializedFile>>>> = Arc::new(Mutex::new(Some(vec![Default::default(); (({ let __len_target = { let __field = self.files.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize])));
        { let __range_holder = self.files.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, f) in __range_values.iter().enumerate() {
        (*f.lock().unwrap().as_ref().unwrap()).mutex.lock();
        (*files.lock().unwrap().as_mut().unwrap())[(i) as usize] = serializedFile { name: Arc::new(Mutex::new(Some({ let __selector_holder = (*f.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), base: Arc::new(Mutex::new(Some({ let __selector_holder = (*f.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), size: Arc::new(Mutex::new(Some({ let __selector_holder = (*f.lock().unwrap().as_ref().unwrap()).size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), lines: { let __append_target = Arc::new(Mutex::new(None)).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = (*f.lock().unwrap().as_ref().unwrap()).lines.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }, infos: { let __append_target = Arc::new(Mutex::new(None)).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = (*f.lock().unwrap().as_ref().unwrap()).infos.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }, ..Default::default() };
        (*f.lock().unwrap().as_ref().unwrap()).mutex.unlock();
    } }
        { let new_val = files.clone(); (*ss.lock().unwrap().as_mut().unwrap()).files = new_val; };
        self.mutex.unlock();
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = encode.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = ss.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>)))) };
    }
}

impl GoValueClone for serializedFile {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for serializedFileSet {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
